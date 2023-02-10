//! Derive macro for Deref and DerefMut
//!
//! ```rust
//!
//! fn main() {
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, Item, ItemStruct, Ident, Fields}; 

#[proc_macro_derive(Deref)]
pub fn deref(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
	impl_deref(item)
    } else {
	quote! {
	    compile_error!("not used on struct")
	}.into()
    }

}

fn impl_deref(item: ItemStruct) -> TokenStream {
    let struct_name = &item.ident;

    let (field_type, body) = match item.fields {
	Fields::Named(ref inner) => {
	    if inner.named.len() != 1 {
		return quote! {
		    compile_error!("named struct must have only one field")
		}.into();
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
		}.into();
	    }

	    let field = inner.unnamed.first().unwrap().clone();
	    let body = quote! {
		&self.0
	    };
	    (field.ty, body)
	}
	Fields::Unit => {
	    return quote! {
		compile_error!("must not be unit struct")
	    }.into();
	}
    };
	    
    quote! {

	impl std::ops::Deref for #struct_name {
	    type Target = #field_type;
	    fn deref(&self) -> &Self::Target {
		#body
	    }
	}

    }.into()

}
