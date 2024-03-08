
//! Autogenerated weights for `sygma_access_segregator`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/standalone-node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// sygma_access_segregator
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// access_weight.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `sygma_access_segregator`.
pub struct SygmaWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> super::WeightInfo for SygmaWeightInfo<T> {
	/// Storage: SygmaAccessSegregator ExtrinsicAccess (r:0 w:1)
	/// Proof Skipped: SygmaAccessSegregator ExtrinsicAccess (max_values: None, max_size: None, mode: Measured)
	fn grant_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
