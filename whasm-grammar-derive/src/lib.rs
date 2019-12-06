extern crate proc_macro;

use proc_macro_error::proc_macro_error;
use proc_macro::TokenStream;
use syn::{ parse_macro_input, DeriveInput };

#[proc_macro_error]
#[proc_macro_derive(Grammar, attributes(discriminant, forward, sized))]
pub fn wasm_grammar_derive(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    impl_wasm_grammar(derive).into()
}

mod implementation;
mod structure;
mod enumeration;
mod attributes;
mod fields;
mod crate_name;

use crate::implementation::*;