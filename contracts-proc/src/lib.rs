extern crate proc_macro;

use proc_macro::TokenStream;
use std::mem;
use std::path::Path;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use proc_macro2::Ident;
use quote::__private::ext::RepToTokensExt;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    FnArg, ImplItem, Index, ItemImpl, Lit, LitStr, Pat, PatType, Result, ReturnType, Token, Type,
};

const ERC721_CALL_TRAITS: &[(&str, &str)] = &[
    ("Update", "ERC721UpdateVirtual")
];

#[proc_macro_derive(ERC721Virtual, attributes(set))]
pub fn erc721_virtual_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    // let mut input = parse_macro_input!(input as ItemImpl);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut set_attrs = vec![];
    for attr in mem::take(&mut input.attrs) {
        if !attr.path().is_ident("set") {
            input.attrs.push(attr);
            continue;
        }

        let upd_type: SetAttr = match attr.parse_args() {
            Ok(contents) => contents,
            Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
        };
        set_attrs.push(upd_type);
    }

    
    ERC721_CALL_TRAITS.iter().map(|&(c, t)|{
        let matched_set_attr = set_attrs.iter().find(|&attr| attr.call.is_ident(c));
        if let Some(set_attr) = matched_set_attr {
            
        } else { 
            
        }
    })
    
    let call_overrides = quote! {};
    let struct_overrides = quote! {};
    let upd = &set_attrs[0].call;
    let upd_overr = &set_attrs[0].overr;

    let expanded = quote! {
        impl #impl_generics ERC721Virtual for #name #ty_generics #where_clause {
            type #upd = #upd_overr<Base::Update>;
        }

        pub struct #upd_overr<V: ERC721UpdateVirtual>(V);

        // #(#upd_types, )*

        // impl<Base: ERC721Virtual> ERC721Virtual for NoWayOverride<Base> {
        //     type Update = NoWayUpdateOverride<Base::Update>;
        // }
        //
        // pub struct NoWayUpdateOverride<V: ERC721UpdateVirtual>(V);
    };

    TokenStream::from(expanded)
}

struct SetAttr {
    call: syn::Path,
    overr: syn::Path,
}

impl Parse for SetAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args: Punctuated::<syn::Path, syn::Token![=]> = Punctuated::parse_separated_nonempty(input)?;
        let mut iter = args.into_iter();
        Ok(Self { call: iter.next().unwrap(), overr: iter.next().unwrap() })
    }
}