//! # EVM backends
//!
//! Backends store state information of the VM, and exposes it to runtime.

mod memory;

pub use self::memory::{MemoryBackend, MemoryVicinity, MemoryAccount};

use alloc::vec::Vec;
use primitive_types::{H160, H256, U256};
use core::convert::Infallible;
use crate::{ExitReason, Capture, Transfer, Context};

/// Basic account information.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "with-codec", derive(codec::Encode, codec::Decode))]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Basic {
	/// Account balance.
	pub balance: U256,
	/// Account nonce.
	pub nonce: U256,
}

pub use ethereum::Log;

/// Apply state operation.
#[derive(Clone, Debug)]
pub enum Apply<I> {
	/// Modify or create at address.
	Modify {
		/// Address.
		address: H160,
		/// Basic information of the address.
		basic: Basic,
		/// Code. `None` means leaving it unchanged.
		code: Option<Vec<u8>>,
		/// Storage iterator.
		storage: I,
		/// Whether storage should be wiped empty before applying the storage
		/// iterator.
		reset_storage: bool,
	},
	/// Delete address.
	Delete {
		/// Address.
		address: H160,
	},
}

/// EVM backend.
pub trait Backend {
	/// Gas price.
	fn gas_price(&self) -> U256;
	/// Origin.
	fn origin(&self) -> H160;
	/// Environmental block hash.
	fn block_hash(&self, number: U256) -> H256;
	/// Environmental block number.
	fn block_number(&self) -> U256;
	/// Environmental coinbase.
	fn block_coinbase(&self) -> H160;
	/// Environmental block timestamp.
	fn block_timestamp(&self) -> U256;
	/// Environmental block difficulty.
	fn block_difficulty(&self) -> U256;
	/// Environmental block gas limit.
	fn block_gas_limit(&self) -> U256;
	/// Environmental chain ID.
	fn chain_id(&self) -> U256;

	/// Whether account at address exists.
	fn exists(&self, address: H160) -> bool;
	/// Get basic account information.
	fn basic(&self, address: H160) -> Basic;
	/// Get account code hash.
	fn code_hash(&self, address: H160) -> H256;
	/// Get account code size.
	fn code_size(&self, address: H160) -> usize;
	/// Get account code.
	fn code(&self, address: H160) -> Vec<u8>;
	/// Get storage value of address at index.
	fn storage(&self, address: H160, index: H256) -> H256;

	/// Handles call if it is external
	fn handle_call(
		&self,
		_code_address: H160,
		_transfer: Option<Transfer>,
		_input: Vec<u8>,
		_target_gas: Option<usize>,
		_is_static: bool,
		_take_l64: bool,
		_take_stipend: bool,
		_context: Context,
	) -> Option<Capture<(ExitReason, Vec<u8>), Infallible>>;
}

/// EVM backend that can apply changes.
pub trait ApplyBackend {
	/// Apply given values and logs at backend.
	fn apply<A, I, L>(
		&mut self,
		values: A,
		logs: L,
		delete_empty: bool,
	) where
		A: IntoIterator<Item=Apply<I>>,
		I: IntoIterator<Item=(H256, H256)>,
		L: IntoIterator<Item=Log>;
}
