use core::borrow::BorrowMut;

use alloy_primitives::Address;
use alloy_sol_types::{sol, SolError};
use stylus_sdk::{alloy_primitives::U256, evm, msg, prelude::*};

use crate::erc721::base::{Error, ERC721};

sol_storage! {
    pub struct ERC721Pausable {
        bool paused;
    }
}

sol! {
    /// Emitted when the pause is triggered by `account`.
    event Paused(address account);

    /// Emitted when the pause is lifted by `account`.
    event Unpaused(address account);

    /// The operation failed because the contract is paused.
    error EnforcedPause();

    /// The operation failed because the contract is not paused.
    error ExpectedPause();
}

#[external]
#[borrow(ERC721)]
impl ERC721Pausable {
    /// ERC-721 Pausable implementation
    /// ERC-721 token with pausable token transfers, minting and burning.

    /// Useful for scenarios such as preventing trades until the end of an evaluation
    /// period, or having an emergency switch for freezing all token transfers in the
    /// event of a large bug.
    pub fn paused(&self) -> bool {
        *self.paused
    }

    /// Triggers stopped state.
    ///
    /// Requirements:
    /// - The contract must not be paused.
    fn pause(&mut self) {
        self.paused.set(true);
        evm::log(Paused {
            account: msg::sender(),
        });
    }

    // TODO#q: add override for internal
}