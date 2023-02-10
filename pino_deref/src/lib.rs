//! Derive macro for Deref and DerefMut
//!
//! ```rust
//! use pino_deref::{Deref, DerefMut};
//!
//! #[derive(Deref, DerefMut)]
//! struct Nametag(pub String);
//!
//! fn main() {
//!     let nametag = Nametag("pinosaur".into());
//!     assert_eq!(*nametag, String::from("pinosaur"));
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Item, ItemStruct};

#[proc_macro_derive(Deref)]
pub fn deref(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_deref(item)
    } else {
        quote! {
            compile_error!("not used on struct")
        }
        .into()
    }
}

fn impl_deref(item: ItemStruct) -> TokenStream {
    let struct_name = &item.ident;

    let (field_type, body) = match item.fields {
        Fields::Named(ref inner) => {
            if inner.named.len() != 1 {
                return quote! {
                    compile_error!("named struct must have only one field")
                }
                .into();
            }

            let field = inner.named.first().unwrap().clone();
            let field_name = field.ident;
            let body = quote! {
            &self.#field_name
            };
            (field.ty, body)
        },
        Fields::Unnamed(ref inner) => {
            if inner.unnamed.len() != 1 {
                return quote! {
                    compile_error!("unnamed struct must have only one field")
                }
                .into();
            }

            let field = inner.unnamed.first().unwrap().clone();
            let body = quote! {
            &self.0
            };
            (field.ty, body)
        },
        Fields::Unit => {
            return quote! {
            compile_error!("must not be unit struct")
            }
            .into();
        },
    };

    quote! {

    impl std::ops::Deref for #struct_name {
        type Target = #field_type;
        fn deref(&self) -> &Self::Target {
        #body
        }
    }

    }
    .into()
}

// TODO there is a lot of code duplication, leaving it like this for now in case we add more features in the future that require more different implementations between Deref and DerefMut

#[proc_macro_derive(DerefMut)]
pub fn deref_mut(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_deref_mut(item)
    } else {
        quote! {
            compile_error!("not used on struct")
        }
        .into()
    }
}

fn impl_deref_mut(item: ItemStruct) -> TokenStream {
    let struct_name = &item.ident;

    let body = match item.fields {
        Fields::Named(ref inner) => {
            if inner.named.len() != 1 {
                return quote! {
                    compile_error!("named struct must have only one field")
                }
                .into();
            }

            let field = inner.named.first().unwrap().clone();
            let field_name = field.ident;
            quote! {
            &mut self.#field_name
            }
        },
        Fields::Unnamed(ref inner) => {
            if inner.unnamed.len() != 1 {
                return quote! {
                    compile_error!("unnamed struct must have only one field")
                }
                .into();
            }

            quote! {
            &mut self.0
            }
        },
        Fields::Unit => {
            return quote! {
            compile_error!("must not be unit struct")
            }
            .into();
        },
    };

    quote! {

    impl std::ops::DerefMut for #struct_name {
        fn deref_mut(&mut self) -> &mut Self::Target {
        #body
        }
    }

    }
    .into()
}
