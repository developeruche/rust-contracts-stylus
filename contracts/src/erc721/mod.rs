pub mod extensions;
pub mod base;

use alloy_primitives::{fixed_bytes, Address, FixedBytes, U128, U256};
use stylus_sdk::{
    abi::Bytes, alloy_sol_types::sol, call::Call, evm, msg, prelude::*,
};
use crate::erc721::extensions::{burnable::ERC721Burnable, pausable::ERC721Pausable};

sol_storage!{
    #[entrypoint]
    pub struct ERC721 {
        #[borrow]
        base::ERC721<base::ERC721Base> erc721;
        #[borrow]
        ERC721Burnable<base::ERC721Base> burnable;
        #[borrow]
        ERC721Pausable<base::ERC721Base> pausable;
    }
}

#[external]
#[inherit(ERC721Burnable<base::ERC721Base>)]
#[inherit(ERC721Pausable<base::ERC721Base>)]
#[inherit(base::ERC721<base::ERC721Base>)]
impl ERC721 {}
