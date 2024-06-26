
//! Autogenerated weights for pallet_artists
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro-de-Lois.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("harmonie-dev"), DB CACHE: 1024

// Executed Command:
// target/release/allfeat
// benchmark
// pallet
// --chain=harmonie-dev
// --steps=50
// --repeat=20
// --pallet=pallet_artists
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/artists/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_artists.
pub trait WeightInfo {
	fn register(n: u32, g: u32, a: u32, ) -> Weight;
	fn force_unregister(n: u32, g: u32, a: u32, ) -> Weight;
	fn unregister(n: u32, g: u32, a: u32, ) -> Weight;
	fn update_add_genres(n: u32, ) -> Weight;
	fn update_remove_genres(n: u32, ) -> Weight;
	fn update_clear_genres(n: u32, ) -> Weight;
	fn update_description() -> Weight;
	fn update_add_assets(n: u32, ) -> Weight;
	fn update_remove_assets(n: u32, ) -> Weight;
	fn update_clear_assets(n: u32, ) -> Weight;
	fn update_main_type() -> Weight;
	fn update_extra_types() -> Weight;
}

/// Weights for pallet_artists using the Allfeat node and recommended hardware.
pub struct AllfeatWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AllfeatWeight<T> {
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 128]`.
	/// The range of component `g` is `[0, 5]`.
	/// The range of component `a` is `[0, 64]`.
	fn register(_n: u32, g: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `5749`
		// Minimum execution time: 111_000_000 picoseconds.
		Weight::from_parts(117_685_284, 5749)
			// Standard Error: 407_411
			.saturating_add(Weight::from_parts(781_465, 0).saturating_mul(g.into()))
			// Standard Error: 35_208
			.saturating_add(Weight::from_parts(30_396_209, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `Artists::ArtistOf` (r:0 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 128]`.
	/// The range of component `g` is `[0, 5]`.
	/// The range of component `a` is `[0, 64]`.
	fn force_unregister(n: u32, g: u32, _a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `3610`
		// Minimum execution time: 85_000_000 picoseconds.
		Weight::from_parts(87_711_200, 3610)
			// Standard Error: 1_412
			.saturating_add(Weight::from_parts(3_370, 0).saturating_mul(n.into()))
			// Standard Error: 32_274
			.saturating_add(Weight::from_parts(182_191, 0).saturating_mul(g.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 128]`.
	/// The range of component `g` is `[0, 5]`.
	/// The range of component `a` is `[0, 64]`.
	fn unregister(_n: u32, g: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + a * (32 ±0) + g * (3 ±0) + n * (1 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 113_000_000 picoseconds.
		Weight::from_parts(125_635_935, 5749)
			// Standard Error: 59_766
			.saturating_add(Weight::from_parts(137_509, 0).saturating_mul(g.into()))
			// Standard Error: 5_165
			.saturating_add(Weight::from_parts(48_725, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 4]`.
	fn update_add_genres(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + n * (3 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_054_439, 5749)
			// Standard Error: 15_387
			.saturating_add(Weight::from_parts(247_079, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	fn update_remove_genres(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + n * (3 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 5]`.
	fn update_clear_genres(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + n * (3 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_932_749, 5749)
			// Standard Error: 5_386
			.saturating_add(Weight::from_parts(17_450, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	fn update_description() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `5749`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(64_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 63]`.
	fn update_add_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + n * (32 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_685_994, 5749)
			// Standard Error: 1_922
			.saturating_add(Weight::from_parts(114_929, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 64]`.
	fn update_remove_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + n * (32 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(35_340_914, 5749)
			// Standard Error: 1_834
			.saturating_add(Weight::from_parts(116_430, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 64]`.
	fn update_clear_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + n * (32 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(35_468_068, 5749)
			// Standard Error: 3_886
			.saturating_add(Weight::from_parts(27_726, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	fn update_main_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	fn update_extra_types() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}

impl WeightInfo for () {
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 128]`.
	/// The range of component `g` is `[0, 5]`.
	/// The range of component `a` is `[0, 64]`.
	fn register(_n: u32, g: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `5749`
		// Minimum execution time: 111_000_000 picoseconds.
		Weight::from_parts(117_685_284, 5749)
			// Standard Error: 407_411
			.saturating_add(Weight::from_parts(781_465, 0).saturating_mul(g.into()))
			// Standard Error: 35_208
			.saturating_add(Weight::from_parts(30_396_209, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// Storage: `Artists::ArtistOf` (r:0 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 128]`.
	/// The range of component `g` is `[0, 5]`.
	/// The range of component `a` is `[0, 64]`.
	fn force_unregister(n: u32, g: u32, _a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `3610`
		// Minimum execution time: 85_000_000 picoseconds.
		Weight::from_parts(87_711_200, 3610)
			// Standard Error: 1_412
			.saturating_add(Weight::from_parts(3_370, 0).saturating_mul(n.into()))
			// Standard Error: 32_274
			.saturating_add(Weight::from_parts(182_191, 0).saturating_mul(g.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 128]`.
	/// The range of component `g` is `[0, 5]`.
	/// The range of component `a` is `[0, 64]`.
	fn unregister(_n: u32, g: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + a * (32 ±0) + g * (3 ±0) + n * (1 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 113_000_000 picoseconds.
		Weight::from_parts(125_635_935, 5749)
			// Standard Error: 59_766
			.saturating_add(Weight::from_parts(137_509, 0).saturating_mul(g.into()))
			// Standard Error: 5_165
			.saturating_add(Weight::from_parts(48_725, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 4]`.
	fn update_add_genres(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + n * (3 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_054_439, 5749)
			// Standard Error: 15_387
			.saturating_add(Weight::from_parts(247_079, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	fn update_remove_genres(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + n * (3 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 5]`.
	fn update_clear_genres(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196 + n * (3 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(8_932_749, 5749)
			// Standard Error: 5_386
			.saturating_add(Weight::from_parts(17_450, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	fn update_description() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `5749`
		// Minimum execution time: 63_000_000 picoseconds.
		Weight::from_parts(64_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 63]`.
	fn update_add_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + n * (32 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_685_994, 5749)
			// Standard Error: 1_922
			.saturating_add(Weight::from_parts(114_929, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 64]`.
	fn update_remove_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + n * (32 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(35_340_914, 5749)
			// Standard Error: 1_834
			.saturating_add(Weight::from_parts(116_430, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(145), added: 2620, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 64]`.
	fn update_clear_assets(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296 + n * (32 ±0)`
		//  Estimated: `5749`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(35_468_068, 5749)
			// Standard Error: 3_886
			.saturating_add(Weight::from_parts(27_726, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	fn update_main_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Artists::ArtistOf` (r:1 w:1)
	/// Proof: `Artists::ArtistOf` (`max_values`: None, `max_size`: Some(2284), added: 4759, mode: `MaxEncodedLen`)
	fn update_extra_types() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `5749`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 5749)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
