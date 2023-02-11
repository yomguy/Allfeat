
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `symphonie-01`, CPU: `Intel(R) Xeon(R) Platinum 8168 CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/allfeat
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/symphonie/src/weights/utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 47_725 nanoseconds.
		Weight::from_ref_time(70_232_142)
			// Standard Error: 2_719
			.saturating_add(Weight::from_ref_time(21_553_947).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 25_193 nanoseconds.
		Weight::from_ref_time(25_554_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 47_864 nanoseconds.
		Weight::from_ref_time(58_359_541)
			// Standard Error: 3_038
			.saturating_add(Weight::from_ref_time(22_780_862).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 55_305 nanoseconds.
		Weight::from_ref_time(56_175_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 47_427 nanoseconds.
		Weight::from_ref_time(67_319_487)
			// Standard Error: 2_917
			.saturating_add(Weight::from_ref_time(21_602_070).saturating_mul(c.into()))
	}
}