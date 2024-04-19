use core::marker::PhantomData;
use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};
use crate::erc721::{base::{ERC721UpdateVirtual, ERC721Virtual}, Error, TopLevelStorage};

sol_storage! {
    pub struct ERC721Burnable<V: ERC721Virtual> {
        PhantomData<V> _phantom_data;
    }
}

#[external]
impl<V: ERC721Virtual> ERC721Burnable<V> {
    fn burn(
        storage: &mut impl TopLevelStorage,
        token_id: U256,
    ) -> Result<(), Error> {
        V::Update::call::<V>(storage, Address::ZERO, token_id, msg::sender())?;
        Ok(())
    }
}

pub struct ERC721BurnableOverride<V: ERC721Virtual>(V);

impl<Base: ERC721Virtual> ERC721Virtual for ERC721BurnableOverride<Base> {
    type Update = Base::Update;
}
