// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Nft fractionalization pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame::benchmarking::prelude::*;

use frame::deps::frame_support::assert_ok;
use fungible::{Inspect as InspectFungible, Mutate as MutateFungible};
use nonfungibles_v2::{Create, Mutate};

use frame_system::RawOrigin as SystemOrigin;
use pallet_nfts::{CollectionConfig, CollectionSettings, ItemConfig, MintSettings};

use crate::Pallet as NftFractionalization;

type BalanceOf<T> =
	<<T as Config>::Currency as InspectFungible<<T as SystemConfig>::AccountId>>::Balance;

type CollectionConfigOf<T> =
	CollectionConfig<BalanceOf<T>, BlockNumberFor<T>, <T as Config>::NftCollectionId>;

fn default_collection_config<T: Config>() -> CollectionConfigOf<T>
where
	T::Currency: InspectFungible<T::AccountId>,
{
	CollectionConfig {
		settings: CollectionSettings::all_enabled(),
		max_supply: None,
		mint_settings: MintSettings::default(),
	}
}

fn mint_nft<T: Config>(nft_id: T::NftId) -> (T::AccountId, AccountIdLookupOf<T>)
where
	T::Nfts: Create<T::AccountId, CollectionConfig<BalanceOf<T>, BlockNumberFor<T>, T::NftCollectionId>>
		+ Mutate<T::AccountId, ItemConfig>,
{
	let caller: T::AccountId = whitelisted_caller();
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let ed = T::Currency::minimum_balance();
	let multiplier = BalanceOf::<T>::from(100u8);
	T::Currency::set_balance(&caller, ed * multiplier + T::Deposit::get() * multiplier);

	assert_ok!(T::Nfts::create_collection(&caller, &caller, &default_collection_config::<T>()));
	let collection = T::BenchmarkHelper::collection(0);
	assert_ok!(T::Nfts::mint_into(&collection, &nft_id, &caller, &ItemConfig::default(), true));
	(caller, caller_lookup)
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

#[benchmarks(
	where
		T::Nfts:
			Create<
				T::AccountId,
				CollectionConfig<BalanceOf<T>,
				frame_system::pallet_prelude::BlockNumberFor::<T>,
				T::NftCollectionId>
			>
			+ Mutate<T::AccountId, ItemConfig>,
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn fractionalize() {
		let asset = T::BenchmarkHelper::asset(0);
		let collection = T::BenchmarkHelper::collection(0);
		let nft = T::BenchmarkHelper::nft(0);
		let (caller, caller_lookup) = mint_nft::<T>(nft);

		#[extrinsic_call]
		_(
			SystemOrigin::Signed(caller.clone()),
			collection,
			nft,
			asset.clone(),
			caller_lookup,
			1000u32.into(),
		);

		assert_last_event::<T>(
			Event::NftFractionalized {
				nft_collection: collection,
				nft,
				fractions: 1000u32.into(),
				asset,
				beneficiary: caller,
			}
			.into(),
		);
	}

	#[benchmark]
	fn unify() {
		let asset = T::BenchmarkHelper::asset(0);
		let collection = T::BenchmarkHelper::collection(0);
		let nft = T::BenchmarkHelper::nft(0);
		let (caller, caller_lookup) = mint_nft::<T>(nft);

		assert_ok!(NftFractionalization::<T>::fractionalize(
			SystemOrigin::Signed(caller.clone()).into(),
			collection,
			nft,
			asset.clone(),
			caller_lookup.clone(),
			1000u32.into(),
		));

		#[extrinsic_call]
		_(SystemOrigin::Signed(caller.clone()), collection, nft, asset.clone(), caller_lookup);

		assert_last_event::<T>(
			Event::NftUnified { nft_collection: collection, nft, asset, beneficiary: caller }
				.into(),
		);
	}

	impl_benchmark_test_suite!(
		NftFractionalization,
		crate::mock::new_test_ext(),
		crate::mock::Test
	);
}
