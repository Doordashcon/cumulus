#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// Implemented by autogenerated benchmarking code.
pub trait WeightInfo {
	fn set_config_with_resume_threshold() -> Weight;

	fn set_config_with_weights() -> Weight;

	fn set_config_with_restrict_decay() -> Weight;

	fn set_config_with_threshold_weight() -> Weight;

	fn set_config_with_drop_threshold() -> Weight;

	fn set_config_with_suspend_threshold() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_weights() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_resume_threshold() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_restrict_decay() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_threshold_weight() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_drop_threshold() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_suspend_threshold() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

impl WeightInfo for () {
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_weights() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_resume_threshold() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_restrict_decay() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_threshold_weight() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_drop_threshold() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmpQueue QueueConfig (r:1 w:1)
	fn set_config_with_suspend_threshold() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
