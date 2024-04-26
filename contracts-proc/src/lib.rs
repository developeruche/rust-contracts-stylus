extern crate proc_macro;

mod derive_virtual;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated,
};
use crate::derive_virtual::derive_virtual;

const ERC721_CALL_TRAITS: &[(&str, &str)] =
    &[("Update", "ERC721UpdateVirtual")];

#[proc_macro_derive(ERC721Virtual, attributes(set))]
pub fn erc721_derive_virtual(input: TokenStream) -> TokenStream {
    derive_virtual(input, ERC721_CALL_TRAITS)
}


