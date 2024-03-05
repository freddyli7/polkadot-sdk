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

//! Autogenerated weights for `pallet_nft_fractionalization`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_nft_fractionalization
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/nft-fractionalization/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_nft_fractionalization`.
pub trait WeightInfo {
	fn fractionalize() -> Weight;
	fn unify() -> Weight;
}

/// Weights for `pallet_nft_fractionalization` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Nfts::Item` (r:1 w:0)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(479), added: 2954, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	/// Storage: `NftFractionalization::NftToAsset` (r:0 w:1)
	/// Proof: `NftFractionalization::NftToAsset` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	fn fractionalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `609`
		//  Estimated: `4326`
		// Minimum execution time: 164_764_000 picoseconds.
		Weight::from_parts(168_243_000, 4326)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `NftFractionalization::NftToAsset` (r:1 w:1)
	/// Proof: `NftFractionalization::NftToAsset` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(479), added: 2954, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:1)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	fn unify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1422`
		//  Estimated: `4326`
		// Minimum execution time: 120_036_000 picoseconds.
		Weight::from_parts(123_550_000, 4326)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Nfts::Item` (r:1 w:0)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(479), added: 2954, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(140), added: 2615, mode: `MaxEncodedLen`)
	/// Storage: `NftFractionalization::NftToAsset` (r:0 w:1)
	/// Proof: `NftFractionalization::NftToAsset` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	fn fractionalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `609`
		//  Estimated: `4326`
		// Minimum execution time: 164_764_000 picoseconds.
		Weight::from_parts(168_243_000, 4326)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: `NftFractionalization::NftToAsset` (r:1 w:1)
	/// Proof: `NftFractionalization::NftToAsset` (`max_values`: None, `max_size`: Some(92), added: 2567, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:1)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(479), added: 2954, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:1)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(861), added: 3336, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:1)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(89), added: 2564, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	fn unify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1422`
		//  Estimated: `4326`
		// Minimum execution time: 120_036_000 picoseconds.
		Weight::from_parts(123_550_000, 4326)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
}
