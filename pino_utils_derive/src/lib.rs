extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, Item, ItemEnum, Data}; 

#[proc_macro_attribute]
pub fn stringify(attr: TokenStream, input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as Item);

    if let Item::Enum(item) = data {
        gen(item)
    } else {
        quote!{
            compile_error!("not used on enum");
        }.into()
    }
}

fn gen(item: ItemEnum) -> TokenStream {
    let name = &item.ident;

    let arms = item.variants
        .iter()
        .map(|variant| quote!{ #name::#variant => write!(f, "{}", stringify!(#variant))})
        .collect::<Vec<_>>();

    let output = quote!{

        #item

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#arms ,)*
                }
            }
        }
    };
    output.into()
}
