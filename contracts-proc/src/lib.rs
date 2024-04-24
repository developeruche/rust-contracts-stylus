extern crate proc_macro;

use proc_macro::TokenStream;
use std::mem;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use proc_macro2::Ident;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    FnArg, ImplItem, Index, ItemImpl, Lit, LitStr, Pat, PatType, Result, ReturnType, Token, Type,
};


#[proc_macro_derive(ERC721Virtual, attributes(set))]
pub fn erc721_virtual_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    // let mut input = parse_macro_input!(input as ItemImpl);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut upd_types = vec![];
    for attr in mem::take(&mut input.attrs) {
        if !attr.path().is_ident("set") {
            input.attrs.push(attr);
            continue;
        }
        let upd_type: Type = match attr.parse_args() {
            Ok(contents) => contents,
            Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
        };
        upd_types.push(upd_type);
        // contents.
        // for ty in contents.types {
        //     inherits.push(ty);
        // }
    }

    let expanded = quote! {
        impl #impl_generics ERC721Virtual for #name #ty_generics #where_clause {
            type Update = NoWayUpdateOverride<Base::Update>;
        }

        pub struct NoWayUpdateOverride<V: ERC721UpdateVirtual>(V);

        // #(#upd_types, )*

        // impl<Base: ERC721Virtual> ERC721Virtual for NoWayOverride<Base> {
        //     type Update = NoWayUpdateOverride<Base::Update>;
        // }
        //
        // pub struct NoWayUpdateOverride<V: ERC721UpdateVirtual>(V);
    };

    TokenStream::from(expanded)
}

// struct InheritsAttr {
//     types: Punctuated<Type, Token![,]>,
// }
// 
// impl Parse for InheritsAttr {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let types = Punctuated::parse_separated_nonempty(input)?;
//         Ok(Self { types })
//     }
// }