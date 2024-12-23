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

use crate::{self as frame_system, *};
use frame_support::{derive_impl, parameter_types};
use sp_runtime::{type_with_default::TypeWithDefault, BuildStorage, Perbill};

type Block = mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
	}
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const MAX_BLOCK_WEIGHT: Weight = Weight::from_parts(1024, u64::MAX);

parameter_types! {
	pub Version: RuntimeVersion = RuntimeVersion {
		spec_name: alloc::borrow::Cow::Borrowed("test"),
		impl_name: alloc::borrow::Cow::Borrowed("system-test"),
		authoring_version: 1,
		spec_version: 1,
		impl_version: 1,
		apis: sp_version::create_apis_vec!([]),
		transaction_version: 1,
		system_version: 1,
	};
	pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight {
		read: 10,
		write: 100,
	};
	pub RuntimeBlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(Weight::from_parts(10, 0))
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = Weight::from_parts(5, 0);
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAX_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.base_extrinsic = Weight::from_parts(10, 0);
			weights.max_total = Some(MAX_BLOCK_WEIGHT);
			weights.reserved = Some(
				MAX_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAX_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(Perbill::from_percent(0))
		.build_or_panic();
	pub RuntimeBlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(1024, NORMAL_DISPATCH_RATIO);
}

parameter_types! {
	pub static Killed: Vec<u64> = vec![];
}

pub struct RecordKilled;
impl OnKilledAccount<u64> for RecordKilled {
	fn on_killed_account(who: &u64) {
		Killed::mutate(|r| r.push(*who))
	}
}

#[derive(Debug, TypeInfo)]
pub struct DefaultNonceProvider;
impl Get<u64> for DefaultNonceProvider {
	fn get() -> u64 {
		System::block_number()
	}
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl Config for Test {
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type Block = Block;
	type Version = Version;
	type AccountData = u32;
	type OnKilledAccount = RecordKilled;
	type MultiBlockMigrator = MockedMigrator;
	type Nonce = TypeWithDefault<u64, DefaultNonceProvider>;
}

parameter_types! {
	pub static Ongoing: bool = false;
}

pub struct MockedMigrator;
impl frame_support::migrations::MultiStepMigrator for MockedMigrator {
	fn ongoing() -> bool {
		Ongoing::get()
	}

	fn step() -> Weight {
		Weight::zero()
	}
}

pub type SysEvent = frame_system::Event<Test>;

/// A simple call, which one doesn't matter.
pub const CALL: &<Test as Config>::RuntimeCall =
	&RuntimeCall::System(frame_system::Call::set_heap_pages { pages: 0u64 });

/// Create new externalities for `System` module tests.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		RuntimeGenesisConfig::default().build_storage().unwrap().into();
	// Add to each test the initial weight of a block
	ext.execute_with(|| {
		System::register_extra_weight_unchecked(
			<Test as crate::Config>::BlockWeights::get().base_block,
			DispatchClass::Mandatory,
		)
	});
	ext
}
