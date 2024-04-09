use core::{borrow::BorrowMut, marker::PhantomData};

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

use crate::erc721::base;

sol_storage! {
    pub struct ERC721Burnable<T: base::ERC721Override> {
        PhantomData<T> phantom_data;
    }
}

#[external]
#[borrow(base::ERC721<T>)]
impl<T: base::ERC721Override> ERC721Burnable<T> {
    fn burn<S>(storage: &mut S, token_id: U256) -> Result<(), base::Error>
    where
        S: TopLevelStorage + BorrowMut<base::ERC721<T>>,
    {
        T::_update(
            storage.borrow_mut(),
            Address::ZERO,
            token_id,
            msg::sender(),
        )?;
        Ok(())
    }
}
