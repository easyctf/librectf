//! Config is a library that helps developers specify a layered configuration
//! by exporting a custom derive that has extra configuration options.

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use syn::{parse2, Data, DataStruct, DeriveInput, Fields, Lit, Meta, MetaNameValue, NestedMeta};

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
    #[derive(Debug)]
    enum ConfigTypes {
        Arg(String),
        Env(String),
        Key(String),
        Invalid,
    }
    fn meta_to_cfg_type(m: &MetaNameValue) -> ConfigTypes {
        match (m.ident.to_string().as_ref(), &m.lit) {
            ("arg", Lit::Str(s)) => ConfigTypes::Arg(s.value()),
            ("env", Lit::Str(s)) => ConfigTypes::Env(s.value()),
            ("key", Lit::Str(s)) => ConfigTypes::Key(s.value()),
            _ => ConfigTypes::Invalid,
        }
    }
    let fields =
        fields
            .named
            .iter()
            .map(|field| {
                if field.attrs.len() == 0 {
                    panic!("each field must have a #[config()] on it")
                }
                (field, field.attrs.iter().fold(Vec::new(), |mut v, attr| {
                    match attr.interpret_meta() {
                        Some(Meta::List(meta)) => {
                            v.extend(meta.nested.into_iter().map(|meta| {
                                if let NestedMeta::Meta(Meta::NameValue(meta)) = meta {
                                    meta_to_cfg_type(&meta)
                                } else {
                                    panic!("could not parse attribute on field, must be #[config(x = \"y\", ...)]")
                                }
                            }));
                            v
                        }
                        Some(Meta::NameValue(meta)) => {
                            v.push(meta_to_cfg_type(&meta));
                            v
                        },
                        _ => panic!(
                            "could not parse attribute on field, must be #[config(x = \"y\", ...)]"
                        ),
                    }
                }))
            }).collect::<Vec<_>>();

    // begin building token streams
    let mut tokens = TokenStream2::new();
    let mut clap_constructor = TokenStream2::new();
    let mut get_matches_from = TokenStream2::new();
    let mut constructor = TokenStream2::new();

    for (field, metalist) in fields.iter() {
        let ident = field.ident.clone().unwrap();

        let mut match_with_arg = None;
        let mut match_with_env = None;

        for meta in metalist.iter() {
            match meta {
                ConfigTypes::Arg(arg) => {
                    let s = Literal::string(arg);
                    get_matches_from.extend(quote!(#s,));
                    match_with_arg =
                        Some(quote!(.or_else(|| matches.value_of(#s).map(|s| s.to_owned()))));
                    clap_constructor.extend(quote!(
                        .arg(Arg::with_name(#s).long(#s))
                    ))
                }
                ConfigTypes::Env(env) => {
                    let s = Literal::string(env);
                    match_with_env = Some(quote!(.or_else(|| env::var(#s).ok())));
                }
                _ => (),
            }
        }

        let match_with_arg = match_with_arg.unwrap_or_else(|| TokenStream2::new());
        let match_with_env = match_with_env.unwrap_or_else(|| TokenStream2::new());

        let ident_str = Literal::string(ident.to_string().as_ref());
        tokens.extend(quote! {
            let #ident = match None
                                    #match_with_arg
                                    #match_with_env
                        {
                Some(value) => value,
                None => panic!("could not find a config value for {}", #ident_str),
            };
        });

        constructor.extend(quote!(#ident,));
    }

    // put it all together now
    let result = quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #[allow(dead_code)]
            #[allow(missing_docs)]
            pub fn new() -> Result<Self, &'static str> {
                use cfgmacro::clap::{App, Arg};
                use std::env;

                let matches = App::new("openctf")
                                      #clap_constructor
                                      .get_matches_from(vec![ #get_matches_from ]);
                #tokens
                Ok(#ident { #constructor })
            }
        }
    };
    result.into()
}
