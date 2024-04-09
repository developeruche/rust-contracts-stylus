pub mod extensions;
pub mod base;

use alloy_primitives::{fixed_bytes, Address, FixedBytes, U128, U256};
use stylus_sdk::{
    abi::Bytes, alloy_sol_types::sol, call::Call, evm, msg, prelude::*,
};

sol_storage!{
    #[entrypoint]
    pub struct ERC721 {
        #[borrow]
        base::ERC721 erc721;
        #[borrow]
        extensions::burnable::ERC721Burnable burnable;
        #[borrow]
        extensions::pausable::ERC721Pausable pausable;
    }
}

#[external]
#[inherit(extensions::burnable::ERC721Burnable)]
#[inherit(extensions::pausable::ERC721Pausable)]
#[inherit(base::ERC721)]
impl ERC721 {}
