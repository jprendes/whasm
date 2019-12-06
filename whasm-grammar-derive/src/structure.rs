use proc_macro2::{ TokenStream, Ident };
use quote::quote;
use syn::{ DataStruct, Attribute };

use crate::fields::*;
use crate::attributes::*;
use crate::crate_name;

pub fn impl_wasm_grammar_struct(name: &Ident, strct: &DataStruct, attrs: &Vec<Attribute>) -> TokenStream {
    let whasm = crate_name::whasm();
    let fields = impl_wasm_grammar_fields(&strct.fields);
    let make_strct = quote! { #name #fields };
    match get_attribute("sized", attrs) {
        Some(_) => quote! {{
            let size: u32 = #whasm::grammar::deserialize(iter)?;
            let mut iter = iter.take(size as usize);
            let iter = &mut iter;

            let result = #make_strct;

            if let Ok(Byte(_)) = #whasm::grammar::deserialize(iter) {
                Err(#whasm::grammar::Error::RemainingBytesInStream {
                    ident: stringify!(#name).into(),
                })?
            }
            
            result
        }},
        None => quote! {
            #make_strct
        }
    }
}