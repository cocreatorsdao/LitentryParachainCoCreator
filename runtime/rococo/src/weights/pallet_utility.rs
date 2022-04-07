// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-05, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_utility
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	fn batch(c: u32, ) -> Weight {
		(28_564_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((9_833_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	fn as_derivative() -> Weight {
		(13_329_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	fn batch_all(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 49_000
			.saturating_add((10_630_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn dispatch_as() -> Weight {
		(15_989_000 as Weight)
	}
}