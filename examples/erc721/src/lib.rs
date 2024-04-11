#![cfg_attr(not(test), no_main, no_std)]
extern crate alloc;

use alloc::string::String;

use contracts::erc721::{Storage, ERC721};
use stylus_sdk::prelude::{entrypoint, external, sol_storage};

const DECIMALS: u8 = 10;

sol_storage! {
    #[entrypoint]
    struct Token {
        #[borrow]
        ERC721 erc20;
    }
}

#[external]
#[inherit(ERC721)]
impl Token {
    pub fn constructor(&mut self, name: String, symbol: String) {}

    pub fn decimals(&self) -> u8 {
        DECIMALS
    }
}
