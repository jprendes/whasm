use proc_macro2::TokenStream;
use quote::{ quote, quote_spanned };
use syn::{ Field, Fields, FieldsNamed, FieldsUnnamed, spanned::Spanned };

use crate::crate_name;
use crate::attributes::*;

pub fn impl_wasm_grammar_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => impl_wasm_grammar_named_fields(fields),
        Fields::Unnamed(ref fields) => impl_wasm_grammar_unnamed_fields(fields),
        Fields::Unit => impl_wasm_grammar_unit_fields(),
    }.into()
}

fn impl_wasm_grammar_named_fields(fields: &FieldsNamed) -> TokenStream {
    let recurse = fields.named.iter().map(impl_wasm_grammar_named_field);
    quote! {
        {#(#recurse)*}
    }
}

fn impl_wasm_grammar_named_field(field: &Field) -> TokenStream {
    let whasm = crate_name::whasm();
    let name = &field.ident;
    let ty = &field.ty;
    let cond = impl_wasm_grammar_matching_cond(field);
    quote_spanned! {field.span()=> 
        #name: {
            let val: #ty = #whasm::grammar::deserialize(iter)?;
            #cond
            val
        },
    }
}

fn impl_wasm_grammar_unnamed_fields(fields: &FieldsUnnamed) -> TokenStream {
    let recurse = fields.unnamed.iter().map(impl_wasm_grammar_unnamed_field);
    quote! {
        (#(#recurse)*)
    }
}

fn impl_wasm_grammar_unnamed_field(field: &Field) -> TokenStream {
    let whasm = crate_name::whasm();
    let ty = &field.ty;
    let cond = impl_wasm_grammar_matching_cond(field);
    quote_spanned! {field.span()=>
        {
            let val: #ty = #whasm::grammar::deserialize(iter)?;
            #cond
            val
        },
    }
}

fn impl_wasm_grammar_unit_fields() -> TokenStream {
    quote!{ }
}

fn impl_wasm_grammar_matching_cond(field: &Field) -> TokenStream {
    let whasm = crate_name::whasm();
    match get_attribute("matching", &field.attrs) {
        Some(expr) => quote_spanned! {field.span()=>
            match val {
                #expr => (),
                _ => Err(#whasm::grammar::Error::UnsatisfiedMatch {
                    expected: stringify!(#expr).into(),
                    actual: format!("{:?}", val),
                })?
            };
        },
        _ => quote! { }
    }
}