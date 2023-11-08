
//! Autogenerated weights for pallet_migrations
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --pallet
// pallet_migrations
// --extrinsic
// 
// --output
// frame/migrations/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs
// --steps
// 50
// --repeat
// 20

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_migrations.
pub trait WeightInfo {
	fn on_runtime_upgrade() -> Weight;
	fn on_init_base() -> Weight;
	fn on_init_fast_path() -> Weight;
	fn on_init_loop_base() -> Weight;
	fn force_set_cursor() -> Weight;
	fn clear_historic(n: u32,) -> Weight;
}

/// Weights for pallet_migrations using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn clear_historic(_n: u32,) -> Weight {
		Weight::zero() // FAIL-CI
	}
	fn on_init_fast_path() -> Weight {
		Weight::zero()
	}
	/// Storage: MultiBlockMigrations Cursor (r:1 w:0)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn on_runtime_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1495`
		// Minimum execution time: 1_097_000 picoseconds.
		Weight::from_parts(1_183_000, 1495)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: MultiBlockMigrations Cursor (r:1 w:1)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn on_init_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `143`
		//  Estimated: `1495`
		// Minimum execution time: 3_220_000 picoseconds.
		Weight::from_parts(3_463_000, 1495)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MultiBlockMigrations Cursor (r:1 w:0)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn on_init_loop_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1495`
		// Minimum execution time: 1_307_000 picoseconds.
		Weight::from_parts(1_368_000, 1495)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: MultiBlockMigrations Cursor (r:0 w:1)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn force_set_cursor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 866_000 picoseconds.
		Weight::from_parts(946_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: MultiBlockMigrations Cursor (r:1 w:0)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn on_runtime_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1495`
		// Minimum execution time: 1_097_000 picoseconds.
		Weight::from_parts(1_183_000, 1495)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: MultiBlockMigrations Cursor (r:1 w:1)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn on_init_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `143`
		//  Estimated: `1495`
		// Minimum execution time: 3_220_000 picoseconds.
		Weight::from_parts(3_463_000, 1495)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MultiBlockMigrations Cursor (r:1 w:0)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn on_init_loop_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1495`
		// Minimum execution time: 1_307_000 picoseconds.
		Weight::from_parts(1_368_000, 1495)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: MultiBlockMigrations Cursor (r:0 w:1)
	/// Proof: MultiBlockMigrations Cursor (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn force_set_cursor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 866_000 picoseconds.
		Weight::from_parts(946_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn on_init_fast_path() -> Weight {
		Weight::zero()
	}
	fn clear_historic(_n: u32,) -> Weight {
		Weight::zero() // FAIL-CI
	}
}
