use core::marker::PhantomData;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

use crate::erc721::{base::ERC721Virtual, Error, Storage};
use crate::erc721::base::ERC721UpdateVirtual;

sol_storage! {
    pub struct ERC721Burnable<T: ERC721Virtual> {
        PhantomData<T> phantom_data;
    }
}

#[external]
#[restrict_storage_with(impl Storage<T>)]
impl<T: ERC721Virtual> ERC721Burnable<T> {
    fn burn<S>(storage: &mut S, token_id: U256) -> Result<(), Error>
    where
        S: Storage<T>,
    {
        T::Update::call(storage, Address::ZERO, token_id, msg::sender())?;
        Ok(())
    }
}

pub struct ERC721BurnableOverride<T: ERC721Virtual>(PhantomData<T>);

impl<Base: ERC721Virtual> ERC721Virtual for ERC721BurnableOverride<Base> {
    type Update = Base::Update;
}
