
//! Autogenerated weights for `pallet_ddc_clusters`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-16, STEPS: `200`, REPEAT: 1000, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Raids-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/cere
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_ddc_clusters
// --extrinsic
// *
// --steps
// 20
// --repeat
// 50
// --output
// pallets/ddc-clusters/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ddc_clusters.
pub trait WeightInfo {
	fn create_cluster() -> Weight;
	fn add_node() -> Weight;
	fn remove_node() -> Weight;
	fn set_cluster_params() -> Weight;
	fn set_cluster_gov_params() -> Weight;
}

/// Weight functions for `pallet_ddc_clusters`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DdcClusters Clusters (r:1 w:1)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn create_cluster() -> Weight {
		// Minimum execution time: 14_000 nanoseconds.
		Weight::from_ref_time(15_000_000u64)
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(2u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcNodes CDNNodes (r:1 w:1)
	// Storage: DdcStaking Nodes (r:1 w:0)
	// Storage: DdcStaking CDNs (r:1 w:0)
	// Storage: DdcStaking Storages (r:1 w:0)
	// Storage: DdcStaking Bonded (r:1 w:0)
	// Storage: DdcStaking Ledger (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Contracts ContractInfoOf (r:1 w:1)
	// Storage: Contracts CodeStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System EventTopics (r:2 w:2)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	// Storage: unknown [0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3] (r:1 w:0)
	// Storage: unknown [0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957] (r:1 w:0)
	fn add_node() -> Weight {
		// Minimum execution time: 307_000 nanoseconds.
		Weight::from_ref_time(354_000_000u64)
			.saturating_add(T::DbWeight::get().reads(15u64))
			.saturating_add(T::DbWeight::get().writes(5u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcNodes CDNNodes (r:1 w:1)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	fn remove_node() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(24_000_000u64)
			.saturating_add(T::DbWeight::get().reads(2u64))
			.saturating_add(T::DbWeight::get().writes(2u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:1)
	fn set_cluster_params() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_000_000u64)
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn set_cluster_gov_params() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_000_000u64)
			.saturating_add(T::DbWeight::get().reads(1u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: DdcClusters Clusters (r:1 w:1)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn create_cluster() -> Weight {
		// Minimum execution time: 14_000 nanoseconds.
		Weight::from_ref_time(15_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(1u64))
			.saturating_add(RocksDbWeight::get().writes(2u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcNodes CDNNodes (r:1 w:1)
	// Storage: DdcStaking Nodes (r:1 w:0)
	// Storage: DdcStaking CDNs (r:1 w:0)
	// Storage: DdcStaking Storages (r:1 w:0)
	// Storage: DdcStaking Bonded (r:1 w:0)
	// Storage: DdcStaking Ledger (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Contracts ContractInfoOf (r:1 w:1)
	// Storage: Contracts CodeStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System EventTopics (r:2 w:2)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	// Storage: unknown [0x89eb0d6a8a691dae2cd15ed0369931ce0a949ecafa5c3f93f8121833646e15c3] (r:1 w:0)
	// Storage: unknown [0xc3ad1d87683b6ac25f2e809346840d7a7ed0c05653ee606dba68aba3bdb5d957] (r:1 w:0)
	fn add_node() -> Weight {
		// Minimum execution time: 307_000 nanoseconds.
		Weight::from_ref_time(354_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(15u64))
			.saturating_add(RocksDbWeight::get().writes(5u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcNodes CDNNodes (r:1 w:1)
	// Storage: DdcClusters ClustersNodes (r:0 w:1)
	fn remove_node() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(24_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(2u64))
			.saturating_add(RocksDbWeight::get().writes(2u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:1)
	fn set_cluster_params() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(1u64))
			.saturating_add(RocksDbWeight::get().writes(1u64))
	}
	// Storage: DdcClusters Clusters (r:1 w:0)
	// Storage: DdcClusters ClustersGovParams (r:0 w:1)
	fn set_cluster_gov_params() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(1u64))
			.saturating_add(RocksDbWeight::get().writes(1u64))
	}
}