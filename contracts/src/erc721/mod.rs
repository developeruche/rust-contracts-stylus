pub mod base;
pub mod extensions;

use core::borrow::BorrowMut;
use core::ops::Deref;

use alloy_primitives::{fixed_bytes, Address, FixedBytes, U128, U256};
use stylus_sdk::{
    abi::Bytes, alloy_sol_types::sol, call::Call, evm, msg, prelude::*,
};

use crate::erc721::{
    base::ERC721Virtual,
    extensions::{burnable::ERC721Burnable, pausable::ERC721Pausable},
};
use crate::erc721::base::ERC721Base;
use crate::erc721::extensions::burnable::ERC721BurnableOverride;
use crate::erc721::extensions::pausable::ERC721PausableOverride;

pub trait Storage<T: ERC721Virtual>:
    TopLevelStorage
    + BorrowMut<ERC721Burnable<T>>
    + BorrowMut<ERC721Pausable<T>>
    + BorrowMut<base::ERC721<T>>
{
    fn erc721_mut(&mut self) -> &mut base::ERC721<T>{
        self.borrow_mut()
    }
    
    fn erc721(&self) -> &base::ERC721<T>{
        self.borrow()
    }
}

type ERC721Override = ERC721BurnableOverride<ERC721PausableOverride<ERC721Base>>;

impl Storage<ERC721Override> for ERC721 {}

sol_storage! {
    #[entrypoint]
    pub struct ERC721 {
        #[borrow]
        base::ERC721<ERC721Override> erc721;
        #[borrow]
        ERC721Burnable<ERC721Override> burnable;
        #[borrow]
        ERC721Pausable<ERC721Override> pausable;
    }
}

#[external]
#[inherit(ERC721Burnable<ERC721Override>)]
#[inherit(ERC721Pausable<ERC721Override>)]
#[inherit(base::ERC721<ERC721Override>)]
#[restrict_storage_with(impl Storage<ERC721Override>)]
impl ERC721 {}
