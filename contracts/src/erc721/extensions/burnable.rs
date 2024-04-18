use core::marker::PhantomData;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

use crate::erc721::{
    base::{ERC721UpdateVirtual, ERC721Virtual},
    Error, TopLevelStorage,
};

sol_storage! {
    pub struct ERC721Burnable<T: ERC721Virtual> {
        PhantomData<T> phantom_data;
    }
}

#[external]
impl<T: ERC721Virtual> ERC721Burnable<T> {
    fn burn(
        storage: &mut impl TopLevelStorage,
        token_id: U256,
    ) -> Result<(), Error> {
        T::Update::call::<T>(storage, Address::ZERO, token_id, msg::sender())?;
        Ok(())
    }
}

pub struct ERC721BurnableOverride<T: ERC721Virtual>(PhantomData<T>);

impl<Base: ERC721Virtual> ERC721Virtual for ERC721BurnableOverride<Base> {
    type Update = Base::Update;
}
