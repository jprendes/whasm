use proc_macro2::TokenStream;
use quote::{ quote, quote_spanned };
use syn::{ Field, Fields, FieldsNamed, FieldsUnnamed, spanned::Spanned };

use crate::crate_name;

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
    quote_spanned! {field.span()=>
        #name: #whasm::grammar::deserialize(iter)?,
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
    quote_spanned! {field.span()=>
        #whasm::grammar::deserialize(iter)?,
    }
}

fn impl_wasm_grammar_unit_fields() -> TokenStream {
    quote!{ }
}