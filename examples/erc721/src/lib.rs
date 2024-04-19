#![cfg_attr(not(test), no_main, no_std)]
extern crate alloc;

use alloy_primitives::{Address, U256};
use contracts::erc721::{
    base::{
        ERC721Base, ERC721BaseOverride, ERC721UpdateVirtual, ERC721Virtual,
    },
    extensions::{
        ERC721Burnable, ERC721BurnableOverride, ERC721Pausable,
        ERC721PausableOverride,
    },
    Error,
};
use stylus_sdk::{alloy_sol_types::sol, evm, prelude::*};

type Override = NoWayOverride<
    ERC721BurnableOverride<ERC721PausableOverride<ERC721BaseOverride>>,
>;

sol! {
    /// Emitted when life is not doomed and there is a way.
    event ThereIsWay();

    /// The operation failed because there is no way. Like end of the world.
    #[derive(Debug)]
    error NoWay();
}

sol_storage! {
    #[entrypoint]
    struct NoWayNft {
        bool _is_there_a_way;

        #[borrow]
        ERC721Base<Override> erc721;
        #[borrow]
        ERC721Burnable<Override> burnable;
        #[borrow]
        ERC721Pausable<Override> pausable;
    }
}

#[external]
#[inherit(ERC721Burnable<Override>)]
#[inherit(ERC721Pausable<Override>)]
#[inherit(ERC721Base<Override>)]
impl NoWayNft {
    fn is_there_a_way(&self) -> bool {
        *self._is_there_a_way
    }

    fn no_way(&mut self) {
        self._is_there_a_way.set(false);
    }

    fn there_is_a_way(&mut self) {
        self._is_there_a_way.set(true);
    }
}

pub struct NoWayOverride<V: ERC721Virtual>(V);

impl<Base: ERC721Virtual> ERC721Virtual for NoWayOverride<Base> {
    type Update = NoWayUpdateOverride<Base::Update>;
}

pub struct NoWayUpdateOverride<V: ERC721UpdateVirtual>(V);

impl<Base: ERC721UpdateVirtual> ERC721UpdateVirtual
    for NoWayUpdateOverride<Base>
{
    fn call<V: ERC721Virtual>(
        storage: &mut impl TopLevelStorage,
        to: Address,
        token_id: U256,
        auth: Address,
    ) -> Result<Address, Error> {
        let storage: &mut NoWayNft = storage.get_storage();
        if storage.is_there_a_way() {
            evm::log(ThereIsWay {});
            Base::call::<V>(storage, to, token_id, auth)
        } else {
            Err(Error::Custom(NoWay {}.into()))
        }
    }
}
