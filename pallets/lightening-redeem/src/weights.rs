
//! Autogenerated weights for bifrost_lightening_redeem
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `VM-16-3-ubuntu`, CPU: `Intel(R) Xeon(R) Platinum 8374C CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_lightening_redeem
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-kusama/src/weights/bifrost_lightening_redeem.rs
// --template=./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for bifrost_lightening_redeem.
pub trait WeightInfo {
	fn add_ksm_to_pool() -> Weight;
	fn exchange_for_ksm() -> Weight;
	fn edit_exchange_price() -> Weight;
	fn edit_release_per_day() -> Weight;
	fn edit_release_start_and_end_block() -> Weight;
	fn on_initialize() -> Weight;
}

/// Weights for bifrost_lightening_redeem using the Substrate node and recommended hardware.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BifrostWeight<T> {
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_ksm_to_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1489`
		//  Estimated: `11753`
		// Minimum execution time: 74_101_000 picoseconds.
		Weight::from_parts(76_087_000, 11753)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: LighteningRedeem PoolAmount (r:1 w:1)
	/// Proof Skipped: LighteningRedeem PoolAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: LighteningRedeem ExchangePriceDiscount (r:1 w:0)
	/// Proof Skipped: LighteningRedeem ExchangePriceDiscount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:2 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn exchange_for_ksm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2088`
		//  Estimated: `30365`
		// Minimum execution time: 157_064_000 picoseconds.
		Weight::from_parts(160_029_000, 30365)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: LighteningRedeem ExchangePriceDiscount (r:1 w:1)
	/// Proof Skipped: LighteningRedeem ExchangePriceDiscount (max_values: Some(1), max_size: None, mode: Measured)
	fn edit_exchange_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 15_253_000 picoseconds.
		Weight::from_parts(16_074_000, 571)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: LighteningRedeem TokenReleasePerDay (r:1 w:1)
	/// Proof Skipped: LighteningRedeem TokenReleasePerDay (max_values: Some(1), max_size: None, mode: Measured)
	fn edit_release_per_day() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 15_826_000 picoseconds.
		Weight::from_parts(17_895_000, 571)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: LighteningRedeem StartEndReleaseBlock (r:1 w:1)
	/// Proof Skipped: LighteningRedeem StartEndReleaseBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn edit_release_start_and_end_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 15_378_000 picoseconds.
		Weight::from_parts(16_142_000, 571)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: LighteningRedeem StartEndReleaseBlock (r:1 w:0)
	/// Proof Skipped: LighteningRedeem StartEndReleaseBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `603`
		// Minimum execution time: 5_828_000 picoseconds.
		Weight::from_parts(6_089_000, 603)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_ksm_to_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1489`
		//  Estimated: `11753`
		// Minimum execution time: 74_101_000 picoseconds.
		Weight::from_parts(76_087_000, 11753)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: LighteningRedeem PoolAmount (r:1 w:1)
	/// Proof Skipped: LighteningRedeem PoolAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: LighteningRedeem ExchangePriceDiscount (r:1 w:0)
	/// Proof Skipped: LighteningRedeem ExchangePriceDiscount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:2 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn exchange_for_ksm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2088`
		//  Estimated: `30365`
		// Minimum execution time: 157_064_000 picoseconds.
		Weight::from_parts(160_029_000, 30365)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: LighteningRedeem ExchangePriceDiscount (r:1 w:1)
	/// Proof Skipped: LighteningRedeem ExchangePriceDiscount (max_values: Some(1), max_size: None, mode: Measured)
	fn edit_exchange_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 15_253_000 picoseconds.
		Weight::from_parts(16_074_000, 571)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: LighteningRedeem TokenReleasePerDay (r:1 w:1)
	/// Proof Skipped: LighteningRedeem TokenReleasePerDay (max_values: Some(1), max_size: None, mode: Measured)
	fn edit_release_per_day() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 15_826_000 picoseconds.
		Weight::from_parts(17_895_000, 571)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: LighteningRedeem StartEndReleaseBlock (r:1 w:1)
	/// Proof Skipped: LighteningRedeem StartEndReleaseBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn edit_release_start_and_end_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 15_378_000 picoseconds.
		Weight::from_parts(16_142_000, 571)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: LighteningRedeem StartEndReleaseBlock (r:1 w:0)
	/// Proof Skipped: LighteningRedeem StartEndReleaseBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `603`
		// Minimum execution time: 5_828_000 picoseconds.
		Weight::from_parts(6_089_000, 603)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
