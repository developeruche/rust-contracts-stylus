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
        extensions::burn::ERC721Burnable burnable;
    }
}

#[external]
#[inherit(extensions::burn::ERC721Burnable, base::ERC721)]
impl ERC721 {}
