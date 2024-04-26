use core::marker::PhantomData;

use alloy_primitives::Address;
use contracts_proc::ERC721Virtual;
use stylus_sdk::{alloy_primitives::U256, evm, msg, prelude::*};

use crate::{
    erc721::{
        base::{ERC721UpdateVirtual, ERC721Virtual},
        Error, TopLevelStorage,
    },
    utils::{pausable::Pausable, pausable},
};

sol_storage! {
    pub struct ERC721Pausable<V: ERC721Virtual> {
        Pausable pausable;
        PhantomData<V> _phantom_data;
    }
}

#[external]
#[inherit(Pausable)]
impl<V: ERC721Virtual> ERC721Pausable<V> {}

#[derive(ERC721Virtual)]
#[set(Update = ERC721PausableUpdateOverride)]
pub struct ERC721PausableOverride<V: ERC721Virtual>(V);

impl<Base: ERC721UpdateVirtual> ERC721UpdateVirtual
    for ERC721PausableUpdateOverride<Base>
{
    fn call<V: ERC721Virtual>(
        storage: &mut impl TopLevelStorage,
        to: Address,
        token_id: U256,
        auth: Address,
    ) -> Result<Address, Error> {
        let pausable: &mut Pausable = storage.inner_mut();
        pausable.require_not_paused()?;
        Base::call::<V>(storage, to, token_id, auth)
    }
}

// NOTE: error mappings could be optimised (e.g. this_error)
impl From<pausable::Error> for Error {
    fn from(value: pausable::Error) -> Self {
        match value {
            pausable::Error::EnforcedPause(e) => Error::EnforcedPause(e),
            pausable::Error::ExpectedPause(e) => Error::ExpectedPause(e),
        }
    }
}

#[cfg(all(test, feature = "std"))]
pub(crate) mod tests {
    use alloy_primitives::address;
    use once_cell::sync::Lazy;

    use super::*;
    use crate::erc721::{
        base::{ERC721Base},
        tests::{random_token_id, ERC721Override, ERC721},
        TopLevelStorage,
    };

    static ALICE: Lazy<Address> = Lazy::new(msg::sender);

    const BOB: Address = address!("F4EaCDAbEf3c8f1EdE91b6f2A6840bc2E4DD3526");

    #[grip::test]
    fn error_transfer_while_paused(storage: ERC721) {
        let token_id = random_token_id();
        ERC721Base::<ERC721Override>::_mint(storage, *ALICE, token_id)
            .expect("mint a token to Alice");

        let pausable: &mut Pausable = storage.inner_mut();
        pausable.pause();
        let paused = pausable.paused();
        assert!(paused);

        let err = ERC721Base::<ERC721Override>::transfer_from(
            storage, *ALICE, BOB, token_id,
        )
        .expect_err("should not transfer from paused contract");

        assert!(matches!(err, Error::EnforcedPause(_)));
    }
}
