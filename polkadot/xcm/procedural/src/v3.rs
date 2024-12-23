// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Result, Token};

const MAX_JUNCTIONS: usize = 8;

pub mod multilocation {
	use super::*;

	pub fn generate_conversion_functions(input: proc_macro::TokenStream) -> Result<TokenStream> {
		if !input.is_empty() {
			return Err(syn::Error::new(Span::call_site(), "No arguments expected"))
		}

		let from_tuples = generate_conversion_from_tuples(8, 8);

		Ok(quote! {
			#from_tuples
		})
	}

	fn generate_conversion_from_tuples(max_junctions: usize, max_parents: usize) -> TokenStream {
		let mut from_tuples = (0..=max_junctions)
			.map(|num_junctions| {
				let types = (0..num_junctions).map(|i| format_ident!("J{}", i)).collect::<Vec<_>>();
				let idents =
					(0..num_junctions).map(|i| format_ident!("j{}", i)).collect::<Vec<_>>();
				let array_size = num_junctions;
				let interior = if num_junctions == 0 {
					quote!(Junctions::Here)
				} else {
					quote! {
						[#(#idents .into()),*].into()
					}
				};

				let mut from_tuple = quote! {
					impl< #(#types : Into<Junction>,)* > From<( Ancestor, #( #types ),* )> for MultiLocation {
						fn from( ( Ancestor(parents), #(#idents),* ): ( Ancestor, #( #types ),* ) ) -> Self {
							MultiLocation { parents, interior: #interior }
						}
					}

					impl From<[Junction; #array_size]> for MultiLocation {
						fn from(j: [Junction; #array_size]) -> Self {
							let [#(#idents),*] = j;
							MultiLocation { parents: 0, interior: #interior }
						}
					}
				};

				let from_parent_tuples = (0..=max_parents).map(|cur_parents| {
					let parents =
						(0..cur_parents).map(|_| format_ident!("Parent")).collect::<Vec<_>>();
					let underscores =
						(0..cur_parents).map(|_| Token![_](Span::call_site())).collect::<Vec<_>>();

					quote! {
							impl< #(#types : Into<Junction>,)* > From<( #( #parents , )* #( #types , )* )> for MultiLocation {
								fn from( ( #(#underscores,)* #(#idents,)* ): ( #(#parents,)* #(#types,)* ) ) -> Self {
									Self { parents: #cur_parents as u8, interior: #interior }
								}
							}
					}
				});

				from_tuple.extend(from_parent_tuples);
				from_tuple
			})
			.collect::<TokenStream>();

		let from_parent_junctions_tuples = (0..=max_parents).map(|cur_parents| {
			let parents = (0..cur_parents).map(|_| format_ident!("Parent")).collect::<Vec<_>>();
			let underscores =
				(0..cur_parents).map(|_| Token![_](Span::call_site())).collect::<Vec<_>>();

			quote! {
				impl From<( #(#parents,)* Junctions )> for MultiLocation {
					fn from( (#(#underscores,)* junctions): ( #(#parents,)* Junctions ) ) -> Self {
						MultiLocation { parents: #cur_parents as u8, interior: junctions }
					}
				}
			}
		});
		from_tuples.extend(from_parent_junctions_tuples);

		quote! {
			impl From<(Ancestor, Junctions)> for MultiLocation {
				fn from((Ancestor(parents), interior): (Ancestor, Junctions)) -> Self {
					MultiLocation { parents, interior }
				}
			}

			impl From<Junction> for MultiLocation {
				fn from(x: Junction) -> Self {
					MultiLocation { parents: 0, interior: [x].into() }
				}
			}

			#from_tuples
		}
	}
}

pub mod junctions {
	use super::*;

	pub fn generate_conversion_functions(input: proc_macro::TokenStream) -> Result<TokenStream> {
		if !input.is_empty() {
			return Err(syn::Error::new(Span::call_site(), "No arguments expected"))
		}

		// Support up to 8 Parents in a tuple, assuming that most use cases don't go past 8 parents.
		let from_v4 = generate_conversion_from_v4();
		let from_tuples = generate_conversion_from_tuples(MAX_JUNCTIONS);

		Ok(quote! {
			#from_v4
			#from_tuples
		})
	}

	fn generate_conversion_from_tuples(max_junctions: usize) -> TokenStream {
		(1..=max_junctions)
			.map(|num_junctions| {
				let idents =
					(0..num_junctions).map(|i| format_ident!("j{}", i)).collect::<Vec<_>>();
				let types = (0..num_junctions).map(|i| format_ident!("J{}", i)).collect::<Vec<_>>();

				quote! {
					impl<#(#types : Into<Junction>,)*> From<( #(#types,)* )> for Junctions {
						fn from( ( #(#idents,)* ): ( #(#types,)* ) ) -> Self {
							[#(#idents .into()),*].into()
						}
					}
				}
			})
			.collect()
	}

	fn generate_conversion_from_v4() -> TokenStream {
		let match_variants = (0..8u8)
			.map(|current_number| {
				let number_ancestors = current_number + 1;
				let variant = format_ident!("X{}", number_ancestors);
				let idents =
					(0..=current_number).map(|i| format_ident!("j{}", i)).collect::<Vec<_>>();
				let convert = idents
					.iter()
					.map(|ident| {
						quote! { let #ident = core::convert::TryInto::try_into(#ident.clone())?; }
					})
					.collect::<Vec<_>>();

				quote! {
					crate::v4::Junctions::#variant( junctions ) => {
						let [#(#idents),*] = &*junctions;
						#(#convert);*
						[#(#idents),*].into()
					},
				}
			})
			.collect::<TokenStream>();

		quote! {
			impl core::convert::TryFrom<crate::v4::Junctions> for Junctions {
				type Error = ();

				fn try_from(mut new: crate::v4::Junctions) -> core::result::Result<Self, Self::Error> {
					use Junctions::*;
					Ok(match new {
						crate::v4::Junctions::Here => Here,
						#match_variants
					})
				}
			}
		}
	}
}
