extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Model, attributes(column, table_name))]
pub fn model_derive(tokens: TokenStream) -> TokenStream {
    let item: DeriveInput = syn::parse(tokens).unwrap();
    let ident = &item.ident;

    let result = quote! {
        impl #ident {
            pub fn query() {

            }
        }
    };
    result.into()
}
