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
    let mut content = quote! {
        use ::orm::*;
    };
    if let Some((_, ref items)) = item.content {
        for item in items {
            let item = match item {
                Item::Struct(struct_item) => struct_item,
                _ => panic!("only structs should appear in schema!{}"),
            };
            let vis = &item.vis;
            let ident = &item.ident;
            let attrs = &item.attrs;
            let mut tablename = ident.to_string();

            for attr in attrs {
            }

            // fields of the struct
            let mut fcontent = TokenStream2::new();
            let mut implcontent = quote! {
                pub fn model() -> Column<#ident> {
                    Column::default()
                }
            };

            let fields = match &item.fields {
                Fields::Named(FieldsNamed {
                    named: named_fields,
                    ..
                }) => named_fields,
                _ => panic!("structs must used named fields (struct xxx { })"),
            };
            for field in fields {
                let vis = &field.vis;
                let aug_ident = Ident::new(
                    &format!("__orig_{}", field.ident.as_ref().unwrap()),
                    Span::call_site(),
                ); // this is ok because we confirmed it's named
                let ty = &field.ty;
                fcontent.extend(quote! {
                    #vis #aug_ident: #ty,
                });

                let ident = &field.ident.as_ref().unwrap();
                implcontent.extend(quote! {
                    #vis fn #ident() -> Column<#ty> {
                        Column::default()
                    }
                });
            }

            content.extend(quote! {
                #[derive(Default)]
                #vis struct #ident {
                    #fcontent
                }
                impl #ident {
                    #implcontent
                }
                impl Model for #ident {
                }
            });
        }
    }

    // now add the schema object
    let schema_ident = Ident::new(&format!("{}Schema", ident), Span::call_site());
    let schema_def = quote! {
        pub struct #schema_ident {

        }
        #[cfg(feature = "mysql")]
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
