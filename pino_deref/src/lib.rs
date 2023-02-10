//! Derive macro for Deref and DerefMut
//!
//! ```rust
//!
//! fn main() {
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
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
    let struct_name = item.ident;

    match item.fields {
	Fields::Named(inner) => {
	    if inner.named.len() != 1 {
		return quote! {
		    compile_error!("named struct must have only one field")
		}.into();
	    }
	    
	    let field = inner.named.first().unwrap();
	    let field_type = &field.ty;
	    let field_name = &field.ident;
	    
	    quote!{

		impl std::ops::Deref for #struct_name {
		    type Target = #field_type;
		    fn deref(&self) -> &Self::Target {
			&self.#field_name
		    }
		}

	    }.into()
	},
	Fields::Unnamed(inner) => {
	    unimplemented!()
	}
	Fields::Unit => {
	    unimplemented!()
	}
    }

}
