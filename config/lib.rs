//! Config is a library that helps developers specify a layered configuration
//! by exporting a custom derive that has extra configuration options.

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse2, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Config, attributes(config))]
pub fn config_derive(tokens: TokenStream) -> TokenStream {
    let item: DeriveInput = parse2(tokens.into()).unwrap();
    let ident = &item.ident;
    let (impl_generics, ty_generics, where_clause) = &item.generics.split_for_impl();

    // first, let's verify that this is a struct
    let fields = match item.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("#[derive(Config)] should only be used on structs with named fields."),
    };

    // now go through every field of the struct
    for field in fields.named {
        if field.attrs.len() == 0 {
            panic!("each field must have a #[config()] on it")
        }
        for attr in field.attrs {
            let _meta = match attr.interpret_meta() {
                Some(meta) => meta,
                None => panic!(
                    "could not parse attribute on field {}",
                    field.ident.unwrap().to_string()
                ),
            };
        }
    }

    let result = quote!{
        impl #impl_generics #ident #ty_generics #where_clause {
            #[allow(dead_code)]
            #[allow(missing_docs)]
            pub fn new() -> Result<Self, &'static str> {
                unimplemented!()
            }
        }
    };
    result.into()
}
