// TODO: merge into codegen

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{Fields, FieldsNamed, Ident, Item, ItemMod};

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
                _ => panic!("only structs should appear in schema!{}"),
            };
            let vis = &item.vis;
            let ident = &item.ident;

            // fields of the struct
            let mut fcontent = TokenStream2::new();
            let fields = match &item.fields {
                Fields::Named(FieldsNamed {
                    named: named_fields,
                    ..
                }) => named_fields,
                _ => panic!("structs must used named fields (struct xxx { })"),
            };
            for field in fields {
                let vis = &field.vis;
                let ident = &field.ident.as_ref().unwrap(); // this is ok because we confirmed it's named
                let ty = &field.ty;
                fcontent.extend(quote! {
                    #vis #ident: #ty,
                });
            }

            content.extend(quote! {
                #vis struct #ident {
                    #fcontent
                }
            });
        }
    }

    // now add the schema object
    let schema_ident = Ident::new(&format!("{}Schema", ident), Span::call_site());
    let schema_def = quote! {
        pub struct #schema_ident {

        }
        impl<'a> ::orm::Schema<'a, ::orm::MysqlBackend> for #schema_ident {}
    };

    let result = quote! {
        #vis mod #ident {
            #content
            #schema_def
        }
    };
    result.into()
}
