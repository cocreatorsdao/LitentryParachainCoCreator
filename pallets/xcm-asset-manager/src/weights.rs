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

//! Autogenerated weights for pallet_asset_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-15, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_asset_manager
// --extrinsic=*
// --heap-pages=4096
// --steps
// 20
// --repeat
// 50
// --header=./LICENSE_HEADER
// --template=./templates/benchmark/pallet-weight-template.hbs
// --output
// ./pallets/xcm-asset-manager/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
	fn register_foreign_asset_type() -> Weight;
	fn update_foreign_asset_metadata() -> Weight;
	fn set_asset_units_per_second() -> Weight;
	fn add_asset_type() -> Weight;
	fn remove_asset_type() -> Weight;
}

/// Weights for pallet_asset_manager using the Litentry node and recommended hardware.
pub struct LitentryWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for LitentryWeight<T> {
	// Storage: AssetManager AssetTypeId (r:1 w:1)
	// Storage: AssetManager ForeignAssetTracker (r:1 w:1)
	// Storage: AssetManager AssetIdType (r:0 w:1)
	// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	fn register_foreign_asset_type() -> Weight {
		(26_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	fn update_foreign_asset_metadata() -> Weight {
		(17_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: AssetManager AssetIdUnitsPerSecond (r:0 w:1)
	fn set_asset_units_per_second() -> Weight {
		(16_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:1 w:1)
	fn add_asset_type() -> Weight {
		(21_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:2 w:1)
	// Storage: AssetManager AssetIdType (r:0 w:1)
	fn remove_asset_type() -> Weight {
		(29_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetManager AssetTypeId (r:1 w:1)
	// Storage: AssetManager ForeignAssetTracker (r:1 w:1)
	// Storage: AssetManager AssetIdType (r:0 w:1)
	// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	fn register_foreign_asset_type() -> Weight {
		(26_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	fn update_foreign_asset_metadata() -> Weight {
		(17_300_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:0)
	// Storage: AssetManager AssetIdUnitsPerSecond (r:0 w:1)
	fn set_asset_units_per_second() -> Weight {
		(16_300_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetManager AssetIdType (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:1 w:1)
	fn add_asset_type() -> Weight {
		(21_200_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: AssetManager AssetTypeId (r:2 w:1)
	// Storage: AssetManager AssetIdType (r:0 w:1)
	fn remove_asset_type() -> Weight {
		(29_600_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
