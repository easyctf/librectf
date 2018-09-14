// TODO: merge into codegen

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Item, ItemMod};

#[proc_macro_attribute]
pub fn schema_attr(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemMod = syn::parse(item).expect("#[schema_attr] is expecting a mod");
    let vis = &item.vis;
    let ident = &item.ident;

    // all of the contents should be #[model]s or #[table]s
    let mut content = TokenStream2::new();
    if let Some((_, ref items)) = item.content {
        for item in items {
            let item = match item {
                Item::Struct(struct_item) => struct_item,
                _ => panic!("only structs should appear in schema!{}")
            };
            let vis = &item.vis;
            let ident = &item.ident;
            content.extend(quote! {
                #vis struct #ident {}
            });
        }
    }

    let result = quote! {
        #vis mod #ident {
            #content
        }
    };
    result.into()
}
