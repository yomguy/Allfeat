// This file is part of Allfeat.

// Copyright (C) 2022-2024 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Autogenerated weights for pallet_timestamp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-08-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `debian-32gb-fsn1-1`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("harmonie-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/allfeat
// benchmark
// pallet
// --chain=harmonie-dev
// --steps=50
// --repeat=20
// --pallet=pallet_timestamp
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/shared/src/weights/timestamp.rs
// --header=./HEADER
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_timestamp.
pub trait WeightInfo {
	fn set() -> Weight;
	fn on_finalize() -> Weight;
}

/// Weights for pallet_timestamp using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for AllfeatWeight<T> {
	/// Storage: `Timestamp::Now` (r:1 w:1)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Babe::CurrentSlot` (r:1 w:0)
	/// Proof: `Babe::CurrentSlot` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211`
		//  Estimated: `1493`
		// Minimum execution time: 13_920_000 picoseconds.
		Weight::from_parts(14_481_000, 1493)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `0`
		// Minimum execution time: 7_400_000 picoseconds.
		Weight::from_parts(7_601_000, 0)
	}
}
