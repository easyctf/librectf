extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Model, attributes(column, table_name))]
pub fn model_derive(_tokens: TokenStream) -> TokenStream {
    let result = quote! {
        
    };
    result.into()
}
