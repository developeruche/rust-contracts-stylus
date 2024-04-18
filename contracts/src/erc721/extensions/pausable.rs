use core::marker::PhantomData;
use alloy_primitives::Address;
use alloy_sol_types::sol;
use stylus_sdk::{alloy_primitives::U256, evm, msg, prelude::*};
use crate::erc721::{base::ERC721Virtual, Error, TopLevelStorage};
use crate::erc721::base::ERC721UpdateVirtual;

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

#[external]
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

// TODO#q: should we add derive with auto implementation ERC721Virtual?
pub struct ERC721PausableOverride<T: ERC721Virtual>(PhantomData<T>);

impl<T: ERC721Virtual> ERC721Virtual for ERC721PausableOverride<T> {
    type Update = ERC721PausableUpdateOverride<T::Update>;
}

pub struct ERC721PausableUpdateOverride<T: ERC721UpdateVirtual>(PhantomData<T>);

impl<Base: ERC721UpdateVirtual> ERC721UpdateVirtual for ERC721PausableUpdateOverride<Base> {
    fn call<This: ERC721Virtual>(
        storage: &mut impl TopLevelStorage,
        to: Address,
        token_id: U256,
        auth: Address,
    ) -> Result<Address, Error> {
        let pausable: &mut ERC721Pausable<This> = storage.get_storage();
        pausable.require_not_paused()?;
        Base::call::<This>(storage, to, token_id, auth)
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use alloy_primitives::address;
    use once_cell::sync::Lazy;

    use super::*;
    use crate::erc721::{
        base::ERC721Base, tests::random_token_id, TopLevelStorage, tests::ERC721,
    };
    use crate::erc721::tests::ERC721Override;

    static ALICE: Lazy<Address> = Lazy::new(msg::sender);

    const BOB: Address = address!("F4EaCDAbEf3c8f1EdE91b6f2A6840bc2E4DD3526");

    #[grip::test]
    fn error_transfer_while_paused(storage: ERC721) {
        let token_id = random_token_id();
        ERC721Base::<ERC721Override>::_mint(storage, *ALICE, token_id)
            .expect("mint a token to Alice");

        let pausable: &mut ERC721Pausable<ERC721Override> = storage.get_storage();
        pausable.pause();
        let paused = pausable.paused();
        assert!(paused);

        let err = ERC721Base::<ERC721Override>::transfer_from(storage, *ALICE, BOB, token_id)
            .expect_err("should not transfer from paused contract");

        assert!(matches!(err, Error::EnforcedPause(_)));
    }
}
