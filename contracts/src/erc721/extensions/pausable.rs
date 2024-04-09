use core::{borrow::BorrowMut, marker::PhantomData};

use alloy_primitives::Address;
use alloy_sol_types::{sol, SolError};
use stylus_sdk::{alloy_primitives::U256, evm, msg, prelude::*};

use crate::erc721::{base, base::{ERC721Virtual, Transfer, ERC721}, Storage};

sol_storage! {
    pub struct ERC721Pausable<T> {
        bool paused;
        PhantomData<T> phantom_data;
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
#[restrict_storage_with(impl Storage<T>)]
impl<T: base::ERC721Virtual> ERC721Pausable<T> {
    /// ERC-721 Pausable implementation
    /// ERC-721 token with pausable token transfers, minting and burning.

    /// Useful for scenarios such as preventing trades until the end of an
    /// evaluation period, or having an emergency switch for freezing all
    /// token transfers in the event of a large bug.
    pub fn paused(&self) -> bool {
        *self.paused
    }

    /// Triggers stopped state.
    ///
    /// Requirements:
    /// - The contract must not be paused.
    fn pause(&mut self) {
        self.paused.set(true);
        evm::log(Paused { account: msg::sender() });
    }

    fn require_not_paused(&self) -> Result<(), Error> {
        if self.paused() {
            Err(EnforcedPause {}.into())
        } else {
            Ok(())
        }
    }
}

pub struct ERC721PausableOverride<T: ERC721Virtual>(PhantomData<T>);

impl<Base: ERC721Virtual> ERC721Virtual for ERC721PausableOverride<Base> {
    fn _update<This: ERC721Virtual>(
        storage: &mut impl Storage<This>,
        to: Address,
        token_id: U256,
        auth: Address,
    ) -> Result<Address, crate::erc721::base::Error>
    {
        Base::_update(storage, to, token_id, auth)
    }
}
