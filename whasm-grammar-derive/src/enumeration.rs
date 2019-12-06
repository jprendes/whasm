use proc_macro_error::abort;
use proc_macro2::{ TokenStream, Ident };
use quote::{ quote, quote_spanned };
use syn::{ DataEnum, Variant, Attribute, spanned::Spanned };

use crate::crate_name;
use crate::fields::*;
use crate::attributes::*;

pub fn impl_wasm_grammar_enum(name: &Ident, enm: &DataEnum, attrs: &Vec<Attribute>) -> TokenStream {
    let whasm = crate_name::whasm();
    let recurse = enm.variants.iter().enumerate().map(|(i, v)| {
        impl_wasm_grammar_variant(i, name, v)
    });
    let get_discriminant = quote!{
        let #whasm::grammar::Byte(byte) = #whasm::grammar::deserialize(iter)?;
    };
    let make_variant = quote! {
        match byte {
            #(#recurse)*
            id => Err(#whasm::grammar::Error::InvalidEnumDiscriminant {
                ident: stringify!(#name).into(),
                discriminant: format!("{}", #whasm::grammar::Byte(id))
            })?
        }
    };
    match get_attribute("sized", attrs) {
        Some(_) => quote! {{
            #get_discriminant
            let size: u32 = #whasm::grammar::deserialize(iter)?;
            let mut iter = iter.take(size as usize);
            let iter = &mut iter;

            let result = #make_variant;

            if let Ok(Byte(_)) = #whasm::grammar::deserialize(iter) {
                Err(#whasm::grammar::Error::RemainingBytesInStream {
                    ident: stringify!(#name).into(),
                })?
            }
            
            result
        }},
        None => quote! {{
            #get_discriminant
            #make_variant
        }}
    }
}

fn impl_wasm_grammar_variant(_i: usize, name: &Ident, variant: &Variant) -> TokenStream {
    let consume = match get_attribute("forward", &variant.attrs) {
        Some(_) => false,
        None => true,
    };
    let discrim = match get_attribute("discriminant", &variant.attrs) {
        Some(tokens) => tokens,
        None if consume == false => quote!{ id },
        _ => abort!(
            variant.span(),
            "Enum variants should have a `discriminant` or a `forward` attribute."
        ),
    };
    let consume = match consume {
        true => quote! { },
        false => quote! {
            let byte = [byte];
            let iter = &mut byte.iter().copied().chain(iter);
        }
    };

    let variant_name = &variant.ident;
    let fields = impl_wasm_grammar_fields(&variant.fields);
    quote_spanned! {variant.span()=>
        #discrim => {
            #consume
            #name::#variant_name #fields
        },
    }
}