// TODO: merge into codegen

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Item, ItemStruct};

#[proc_macro_attribute]
pub fn schema_attr(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    // println!("SCHEMA_ATTR attrs: {:?}", attrs);
    // println!("SCHEMA_ATTR item: {:?}", item);

    let item: Item = syn::parse(item).unwrap();
    let result = quote! {
        #item
    };
    result.into()
}
