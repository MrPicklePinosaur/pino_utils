extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Item, ItemEnum, Ident}; 

// mode sorta useless, since the user can just manipulate the string after however they wish
enum Mode {
    Verbatim,
    LowerCase,
    UpperCase,
}

impl Parse for Mode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mode_string = input.parse::<Ident>()?;
        let mode = if mode_string == "verbatim" {
            Mode::Verbatim
        } else if mode_string == "lowercase" {
            Mode::LowerCase
        } else if mode_string == "uppercase" {
            Mode::UpperCase
        } else {
            Mode::Verbatim
        };
        Ok(mode)
    }
}

#[proc_macro_attribute]
pub fn stringify(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    let parsed_attr = parse_macro_input!(attr as Mode);

    if let Item::Enum(item) = parsed_input {
        gen(item, parsed_attr)
    } else {
        quote!{
            compile_error!("not used on enum");
        }.into()
    }
}

fn gen(item: ItemEnum, mode: Mode) -> TokenStream {
    let name = &item.ident;

    let mut arms = vec![];

    for variant in &item.variants {
        let variant_name = &variant.ident.to_string();
        let modified = match mode {
            Mode::Verbatim => variant_name.to_owned(),
            Mode::LowerCase => variant_name.to_lowercase(),
            Mode::UpperCase => variant_name.to_uppercase(),
        };
        let arm = quote!{ #name::#variant => write!(f, "{}", #modified) };
        arms.push(arm);
    }

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
