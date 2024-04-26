use alloy_sol_types::sol;
use stylus_sdk::{evm, msg, prelude::*};

sol_storage! {
    pub struct Pausable {
        bool _paused;
    }
}

sol! {
    /// Emitted when the pause is triggered by `account`.
    event Paused(address account);

    /// Emitted when the pause is lifted by `account`.
    event Unpaused(address account);

    /// The operation failed because the contract is paused.
    #[derive(Debug)]
    error EnforcedPause();

    /// The operation failed because the contract is not paused.
    #[derive(Debug)]
    error ExpectedPause();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    EnforcedPause(EnforcedPause),
    ExpectedPause(ExpectedPause),
}

#[external]
impl Pausable {
    /// ERC-721 Pausable implementation
    /// ERC-721 token with pausable token transfers, minting and burning.

    /// Useful for scenarios such as preventing trades until the end of an
    /// evaluation period, or having an emergency switch for freezing all
    /// token transfers in the event of a large bug.
    pub fn paused(&self) -> bool {
        *self._paused
    }

    /// Triggers stopped state.
    ///
    /// Requirements:
    /// - The contract must not be paused.
    pub fn pause(&mut self) {
        self._paused.set(true);
        evm::log(Paused { account: msg::sender() });
    }

    pub fn require_not_paused(&self) -> Result<(), Error> {
        if self.paused() {
            Err(EnforcedPause {}.into())
        } else {
            Ok(())
        }
    }
}