use core::borrow::BorrowMut;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

use crate::erc721::base::{Error, ERC721};

sol_storage! {
    pub struct ERC721Burnable {}
}

#[external]
#[borrow(ERC721)]
impl ERC721Burnable {
    fn burn<S>(storage: &mut S, token_id: U256) -> Result<(), Error>
    where
        S: TopLevelStorage + BorrowMut<ERC721> + BorrowMut<ERC721Burnable>,
    {
        let erc721_storage: &mut ERC721 = storage.borrow_mut();
        erc721_storage._update(Address::ZERO, token_id, msg::sender())?;
        Ok(())
    }
}
