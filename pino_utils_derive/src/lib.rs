extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, Item, ItemEnum, Data}; 

#[proc_macro_attribute]
pub fn stringify(attr: TokenStream, input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as Item);

    let output = if let Item::Enum(item) = data {
        let name = &item.ident;
        quote!{
            #[derive(Debug)]
            enum #name {
                
            }

            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        }
    } else {
        quote!{
            compile_error!("not used on enum");
        }
    };

    output.into()
}

