//! Config is a library that helps developers specify a layered configuration
//! by exporting a custom derive that has extra configuration options.

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse2, Data, DeriveInput};

#[proc_macro_derive(Config)]
pub fn config_derive(tokens: TokenStream) -> TokenStream {
    let item: DeriveInput = parse2(tokens.into()).unwrap();

    // first, let's verify that this is a struct
    let _data = match item.data {
        Data::Struct(ref data) => data,
        _ => panic!("#[derive(Config)] should only be used on structs."),
    };

    let result = quote!{};
    result.into()
}
