use core::marker::PhantomData;

use alloy_primitives::Address;
use alloy_sol_types::{sol, SolError};
use stylus_sdk::{alloy_primitives::U256, evm, msg, prelude::*};

use crate::erc721::{
    base::{ERC721Virtual, Error},
    extensions::burnable::ERC721Burnable,
    Storage,
};

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

// NOTE: use shared error not base
// #[derive(SolidityError, Debug)]
// pub enum Error {
//     EnforcedPause(EnforcedPause),
//     ExpectedPause(ExpectedPause),
// }

#[external]
#[restrict_storage_with(impl Storage<T>)]
impl<T: ERC721Virtual> ERC721Pausable<T> {
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
    pub fn pause(&mut self) {
        self.paused.set(true);
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

pub struct ERC721PausableOverride<T: ERC721Virtual>(PhantomData<T>);

impl<Base: ERC721Virtual> ERC721Virtual for ERC721PausableOverride<Base> {
    fn _update<This: ERC721Virtual>(
        storage: &mut impl Storage<This>,
        to: Address,
        token_id: U256,
        auth: Address,
    ) -> Result<Address, crate::erc721::base::Error> {
        let pausable: &ERC721Pausable<This> = storage.borrow();
        pausable.require_not_paused()?;
        Base::_update(storage, to, token_id, auth)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use alloy_primitives::address;
    use once_cell::sync::Lazy;

    use super::*;
    use crate::erc721::{
        base::ERC721Base, tests::random_token_id, Storage, ERC721,
    };

    static ALICE: Lazy<Address> = Lazy::new(msg::sender);

    const BOB: Address = address!("F4EaCDAbEf3c8f1EdE91b6f2A6840bc2E4DD3526");

    #[grip::test]
    fn error_transfer_while_paused(storage: ERC721) {
        let token_id = random_token_id();
        ERC721Base::_mint(storage, *ALICE, token_id)
            .expect("mint a token to Alice");

        storage.pausable_mut().pause();
        let paused = storage.pausable_mut().paused();
        assert!(paused);

        let err = ERC721Base::transfer_from(storage, *ALICE, BOB, token_id)
            .expect_err("should not transfer from paused contract");
        
        assert!(matches!(
            err,
            Error::EnforcedPause(_)
        ));
    }
}
