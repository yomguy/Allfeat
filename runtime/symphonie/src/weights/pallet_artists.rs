
//! Autogenerated weights for pallet_artists
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `symphonie-01`, CPU: `Intel(R) Xeon(R) Platinum 8168 CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_artists
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/pallet_artists.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for pallet_artists using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_artists::weights::WeightInfo for AllfeatWeight<T> {
	/// Storage: Artists Artists (r:1 w:0)
	/// Proof: Artists Artists (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Artists Candidates (r:1 w:1)
	/// Proof: Artists Candidates (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 128]`.
	fn submit_candidacy(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3647`
		// Minimum execution time: 146_028_000 picoseconds.
		Weight::from_parts(148_570_259, 3647)
			// Standard Error: 1_043
			.saturating_add(Weight::from_parts(5_066, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Artists Candidates (r:1 w:1)
	/// Proof: Artists Candidates (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn withdraw_candidacy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3647`
		// Minimum execution time: 137_042_000 picoseconds.
		Weight::from_parts(137_499_000, 3647)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Artists Artists (r:1 w:1)
	/// Proof: Artists Artists (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Artists Candidates (r:1 w:1)
	/// Proof: Artists Candidates (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 128]`.
	fn approve_candidacy(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `121 + n * (1 ±0)`
		//  Estimated: `3647`
		// Minimum execution time: 80_504_000 picoseconds.
		Weight::from_parts(82_184_060, 3647)
			// Standard Error: 604
			.saturating_add(Weight::from_parts(9_866, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Artists Artists (r:1 w:0)
	/// Proof: Artists Artists (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn call_as_artist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3647`
		// Minimum execution time: 62_317_000 picoseconds.
		Weight::from_parts(63_320_000, 3647)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Artists Candidates (r:1 w:0)
	/// Proof: Artists Candidates (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	fn call_as_candidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `3647`
		// Minimum execution time: 64_428_000 picoseconds.
		Weight::from_parts(66_245_000, 3647)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}
