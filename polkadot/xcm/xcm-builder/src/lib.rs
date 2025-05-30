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

//! # XCM-Builder
//!
//! Types and helpers for *building* XCM configuration.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

#[cfg(test)]
mod tests;

#[cfg(feature = "std")]
pub mod test_utils;

mod asset_conversion;
#[allow(deprecated)]
pub use asset_conversion::ConvertedConcreteAssetId;
pub use asset_conversion::{
	AsPrefixedGeneralIndex, ConvertedConcreteId, MatchedConvertedConcreteId,
};

mod asset_exchange;
pub use asset_exchange::SingleAssetExchangeAdapter;

mod barriers;
pub use barriers::{
	AllowExplicitUnpaidExecutionFrom, AllowHrmpNotificationsFromRelayChain,
	AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom,
	AllowUnpaidExecutionFrom, DenyRecursively, DenyReserveTransferToRelayChain, DenyThenTry,
	IsChildSystemParachain, IsParentsOnly, IsSiblingSystemParachain, RespectSuspension,
	TakeWeightCredit, TrailingSetTopicAsId, WithComputedOrigin,
};

mod controller;
pub use controller::{
	Controller, ExecuteController, ExecuteControllerWeightInfo, QueryController,
	QueryControllerWeightInfo, QueryHandler, SendController, SendControllerWeightInfo,
};

mod currency_adapter;
#[allow(deprecated)]
pub use currency_adapter::CurrencyAdapter;

mod fee_handling;
pub use fee_handling::{
	deposit_or_burn_fee, HandleFee, SendXcmFeeToAccount, XcmFeeManagerFromComponents,
};

mod filter_asset_location;
pub use filter_asset_location::{AllAssets, Case, LocationWithAssetFilters, NativeAsset};

mod fungible_adapter;
pub use fungible_adapter::{FungibleAdapter, FungibleMutateAdapter, FungibleTransferAdapter};

mod fungibles_adapter;
pub use fungibles_adapter::{
	AssetChecking, DualMint, FungiblesAdapter, FungiblesMutateAdapter, FungiblesTransferAdapter,
	LocalMint, MintLocation, NoChecking, NonLocalMint,
};

mod location_conversion;
#[allow(deprecated)]
pub use location_conversion::ForeignChainAliasAccount;
pub use location_conversion::{
	Account32Hash, AccountId32Aliases, AccountKey20Aliases, AliasesIntoAccountId32,
	ChildParachainConvertsVia, DescribeAccountId32Terminal, DescribeAccountIdTerminal,
	DescribeAccountKey20Terminal, DescribeAllTerminal, DescribeBodyTerminal, DescribeFamily,
	DescribeLocation, DescribePalletTerminal, DescribeTerminus, DescribeTreasuryVoiceTerminal,
	ExternalConsensusLocationsConverterFor, GlobalConsensusConvertsFor,
	GlobalConsensusParachainConvertsFor, HashedDescription, LocalTreasuryVoiceConvertsVia,
	ParentIsPreset, SiblingParachainConvertsVia,
};

mod matches_location;
pub use matches_location::{
	StartsWith, StartsWithExplicitGlobalConsensus, WithLatestLocationConverter,
};

mod matches_token;
pub use matches_token::IsConcrete;

mod matcher;
pub use matcher::{CreateMatcher, MatchXcm, Matcher};

mod nonfungibles_adapter;
pub use nonfungibles_adapter::{
	NonFungiblesAdapter, NonFungiblesMutateAdapter, NonFungiblesTransferAdapter,
};

mod nonfungible_adapter;
pub use nonfungible_adapter::{
	NonFungibleAdapter, NonFungibleMutateAdapter, NonFungibleTransferAdapter,
};

mod origin_aliases;
pub use origin_aliases::*;

mod origin_conversion;
pub use origin_conversion::{
	BackingToPlurality, ChildParachainAsNative, ChildSystemParachainAsSuperuser, EnsureXcmOrigin,
	LocationAsSuperuser, OriginToPluralityVoice, ParentAsSuperuser, RelayChainAsNative,
	SiblingParachainAsNative, SiblingSystemParachainAsSuperuser, SignedAccountId32AsNative,
	SignedAccountKey20AsNative, SignedToAccountId32, SovereignSignedViaLocation,
};

mod pay;
pub use pay::{FixedLocation, LocatableAssetId, PayAccountId32OnChainOverXcm, PayOverXcm};

mod process_xcm_message;
pub use process_xcm_message::ProcessXcmMessage;

mod routing;
pub use routing::{
	EnsureDecodableXcm, EnsureDelivery, InspectMessageQueues, WithTopicSource, WithUniqueTopic,
};

mod transactional;
pub use transactional::FrameTransactionalProcessor;

#[allow(deprecated)]
pub use universal_exports::UnpaidLocalExporter;
mod universal_exports;
pub use universal_exports::{
	ensure_is_remote, BridgeBlobDispatcher, BridgeMessage, DispatchBlob, DispatchBlobError,
	ExporterFor, HaulBlob, HaulBlobError, HaulBlobExporter, LocalExporter, NetworkExportTable,
	NetworkExportTableItem, SovereignPaidRemoteExporter, UnpaidRemoteExporter,
};

mod weight;
pub use weight::{
	FixedRateOfFungible, FixedWeightBounds, TakeRevenue, UsingComponents, WeightInfoBounds,
};
