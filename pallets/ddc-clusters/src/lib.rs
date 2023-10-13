//! # DDC Nodes Pallet
//!
//! The DDC Clusters pallet is used to manage DDC Clusters
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## GenesisConfig
//!
//! The DDC Clusters pallet depends on the [`GenesisConfig`]. The
//! `GenesisConfig` is optional and allow to set some initial nodes in DDC.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use pallet_ddc_nodes::{NodePubKey, NodeRepository, NodeTrait};
mod cluster;

pub use crate::cluster::{Cluster, ClusterError, ClusterId, ClusterParams};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type NodeRepository: NodeRepository<Self>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		ClusterCreated { cluster_id: ClusterId },
		ClusterNodeAdded { cluster_id: ClusterId, node_pub_key: NodePubKey },
	}

	#[pallet::error]
	pub enum Error<T> {
		ClusterAlreadyExists,
		ClusterDoesNotExist,
		ClusterParamsExceedsLimit,
		AttemptToAddNonExistentNode,
		NodeIsAlreadyAssigned,
	}

	#[pallet::storage]
	#[pallet::getter(fn clusters)]
	pub type Clusters<T: Config> =
		StorageMap<_, Blake2_128Concat, ClusterId, Cluster<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn clusters_nodes)]
	pub type ClustersNodes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		ClusterId,
		Blake2_128Concat,
		NodePubKey,
		bool,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_cluster(
			origin: OriginFor<T>,
			cluster_id: ClusterId,
			cluster_params: ClusterParams,
		) -> DispatchResult {
			let manager_id = ensure_signed(origin)?;
			let cluster = Cluster::new(cluster_id.clone(), manager_id, cluster_params)
				.map_err(|e| Into::<Error<T>>::into(ClusterError::from(e)))?;
			ensure!(!Clusters::<T>::contains_key(&cluster_id), Error::<T>::ClusterAlreadyExists);
			Clusters::<T>::insert(cluster_id.clone(), cluster);
			Self::deposit_event(Event::<T>::ClusterCreated { cluster_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_node(
			origin: OriginFor<T>,
			cluster_id: ClusterId,
			node_pub_key: NodePubKey,
		) -> DispatchResult {
			ensure_signed(origin)?;
			ensure!(Clusters::<T>::contains_key(&cluster_id), Error::<T>::ClusterDoesNotExist);
			let mut node = T::NodeRepository::get(node_pub_key.clone())
				.map_err(|_| Error::<T>::AttemptToAddNonExistentNode)?;
			ensure!(node.get_cluster_id().is_none(), Error::<T>::NodeIsAlreadyAssigned);

			// todo: check that node is authorized by the 'NodeProviderAuthSC' contract
			// todo: check that node provider has a bond for this 'cluster_id' and 'node_pub_key'

			node.set_cluster_id(cluster_id.clone());
			T::NodeRepository::update(node).map_err(|_| Error::<T>::AttemptToAddNonExistentNode)?;
			ClustersNodes::<T>::insert(cluster_id.clone(), node_pub_key.clone(), true);
			Self::deposit_event(Event::<T>::ClusterNodeAdded { cluster_id, node_pub_key });

			Ok(())
		}
	}
}
