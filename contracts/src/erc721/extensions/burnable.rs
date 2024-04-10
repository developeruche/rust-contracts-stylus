use core::marker::PhantomData;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

use crate::erc721::{base::ERC721Virtual, Error, Storage};

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
        T::_update(storage, Address::ZERO, token_id, msg::sender())?;
        Ok(())
    }
}

pub struct ERC721BurnableOverride<T: ERC721Virtual>(PhantomData<T>);

impl<Base: ERC721Virtual> ERC721Virtual for ERC721BurnableOverride<Base> {
    // TODO#q: think about auto implementation
    fn _update<This: ERC721Virtual>(
        storage: &mut impl Storage<This>,
        to: Address,
        token_id: U256,
        auth: Address,
    ) -> Result<Address, Error> {
        Base::_update(storage, to, token_id, auth)
    }
}
