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

//! This calls another contract and returns the returncode and output.

#![no_std]
#![no_main]
include!("../panic_handler.rs");

use uapi::{input, u256_bytes, HostFn, HostFnImpl as api};

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {}

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
	input!(
		512,
		callee_addr: &[u8; 20],
		value: u64,
		callee_input: [u8],
	);

	// the first 4 bytes are reserved for the return code
	let mut output = [0u8; 512];
	let output_ptr = &mut &mut output[4..];

	let code = match api::call(
		uapi::CallFlags::empty(),
		callee_addr,
		u64::MAX,           // How much ref_time to devote for the execution. u64::MAX = use all.
		u64::MAX,           // How much proof_size to devote for the execution. u64::MAX = use all.
		&[u8::MAX; 32],     // No deposit limit.
		&u256_bytes(value), // Value transferred to the contract.
		callee_input,
		Some(output_ptr),
	) {
		Ok(_) => 0,
		Err(code) => code as u32,
	};

	let len = 4 + output_ptr.len();
	output[0..4].copy_from_slice(&code.to_le_bytes());
	api::return_value(uapi::ReturnFlags::empty(), &output[..len]);
}
