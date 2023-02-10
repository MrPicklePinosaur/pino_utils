//! Derive macro to implement display for each variant of enum
//!
//! ```rust
//! use pino_enum_string::enum_string;
//!
//! #[enum_string]
//! enum Weapon {
//!     Red,
//!     Blue,
//!     Green,
//! }
//!
//! fn main() {
//!     assert_eq!("Red", Weapon::Red.to_string());
//!     assert_eq!("Blue", Weapon::Blue.to_string());
//!     assert_eq!("Green", Weapon::Green.to_string());
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, Item, ItemEnum};

// mode sorta useless, since the user can just manipulate the string after however they wish
#[derive(Eq, PartialEq)]
enum Mode {
    Invalid,
    Verbatim,
    LowerCase,
    UpperCase,
}

impl Parse for Mode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // default to verbatim if no option is supplied
        if input.is_empty() {
            return Ok(Mode::Verbatim);
        }

        let mode_string = input.parse::<Ident>()?;
        let mode = if mode_string == "verbatim" {
            Mode::Verbatim
        } else if mode_string == "lowercase" {
            Mode::LowerCase
        } else if mode_string == "uppercase" {
            Mode::UpperCase
        } else {
            Mode::Invalid
        };
        Ok(mode)
    }
}

#[proc_macro_attribute]
pub fn enum_string(attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    let parsed_attr = parse_macro_input!(attr as Mode);
    if parsed_attr == Mode::Invalid {
        return quote! {
            compile_error!("invalid mode")
        }
        .into();
    }

    if let Item::Enum(item) = parsed_input {
        gen(item, parsed_attr)
    } else {
        quote! {
            compile_error!("not used on enum");
        }
        .into()
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
            _ => unreachable!(),
        };
        let arm = quote! { #name::#variant => write!(f, "{}", #modified) };
        arms.push(arm);
    }

    let output = quote! {

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
