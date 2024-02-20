//! Autogenerated weights for pallet_ddc_customers
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-18, STEPS: `50`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bench`, CPU: `DO-Premium-AMD`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/cere
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --pallet=pallet-ddc-customers
// --extrinsic=*
// --steps=50
// --repeat=50
// --template=./.maintain/frame-weight-template.hbs
// --output=pallets/ddc-customers/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ddc_customers.
pub trait WeightInfo {
	fn create_bucket() -> Weight;
	fn deposit() -> Weight;
	fn deposit_extra() -> Weight;
	fn unlock_deposit() -> Weight;
	fn withdraw_unlocked_deposit_update() -> Weight;
	fn withdraw_unlocked_deposit_kill() -> Weight;
	fn set_bucket_params() -> Weight;
	fn remove_bucket() -> Weight;
}

/// Weights for pallet_ddc_customers using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DdcCustomers BucketsCount (r:1 w:1)
	// Proof Skipped: DdcCustomers BucketsCount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Proof Skipped: DdcClusters Clusters (max_values: None, max_size: None, mode: Measured)
	// Storage: DdcCustomers Buckets (r:0 w:1)
	// Proof Skipped: DdcCustomers Buckets (max_values: None, max_size: None, mode: Measured)
	fn create_bucket() -> Weight {
		Weight::from_parts(39_599_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit() -> Weight {
		Weight::from_parts(124_067_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit_extra() -> Weight {
		Weight::from_parts(125_998_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	fn unlock_deposit() -> Weight {
		Weight::from_parts(37_109_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_unlocked_deposit_update() -> Weight {
		Weight::from_parts(130_058_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_unlocked_deposit_kill() -> Weight {
		Weight::from_parts(131_827_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Buckets (r:1 w:1)
	// Proof Skipped: DdcCustomers Buckets (max_values: None, max_size: None, mode: Measured)
	fn set_bucket_params() -> Weight {
		Weight::from_parts(31_820_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DdcCustomers Buckets (r:1 w:1)
	// Proof Skipped: DdcCustomers Buckets (max_values: None, max_size: None, mode: Measured)
	fn remove_bucket() -> Weight {
		Weight::from_parts(34_070_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: DdcCustomers BucketsCount (r:1 w:1)
	// Proof Skipped: DdcCustomers BucketsCount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Proof Skipped: DdcClusters Clusters (max_values: None, max_size: None, mode: Measured)
	// Storage: DdcCustomers Buckets (r:0 w:1)
	// Proof Skipped: DdcCustomers Buckets (max_values: None, max_size: None, mode: Measured)
	fn create_bucket() -> Weight {
		Weight::from_parts(39_599_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit() -> Weight {
		Weight::from_parts(124_067_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit_extra() -> Weight {
		Weight::from_parts(125_998_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	fn unlock_deposit() -> Weight {
		Weight::from_parts(37_109_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_unlocked_deposit_update() -> Weight {
		Weight::from_parts(130_058_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Ledger (r:1 w:1)
	// Proof Skipped: DdcCustomers Ledger (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_unlocked_deposit_kill() -> Weight {
		Weight::from_parts(131_827_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: DdcCustomers Buckets (r:1 w:1)
	// Proof Skipped: DdcCustomers Buckets (max_values: None, max_size: None, mode: Measured)
	fn set_bucket_params() -> Weight {
		Weight::from_parts(31_820_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: DdcCustomers Buckets (r:1 w:1)
	// Proof Skipped: DdcCustomers Buckets (max_values: None, max_size: None, mode: Measured)
	fn remove_bucket() -> Weight {
		Weight::from_parts(34_070_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}