use proc_macro_error::abort;
use proc_macro2::{ TokenStream, Ident };
use quote::quote;
use syn::{ parse_quote, Data, DeriveInput, GenericParam, Generics, Attribute };

use crate::crate_name;
use crate::enumeration::*;
use crate::structure::*;

pub fn impl_wasm_grammar(derive: DeriveInput) -> TokenStream {
    let whasm = crate_name::whasm();
    let name = &derive.ident;
    let data = impl_wasm_grammar_data(name, &derive.data, &derive.attrs);
    let generics = add_trait_bounds(derive.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics #whasm::grammar::Grammar for #name #ty_generics #where_clause {
            fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> #whasm::grammar::Result<Self> {
                Ok(#data)
            }
        }
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    let whasm = crate_name::whasm();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#whasm::grammar::Grammar));
        }
    }
    generics
}

fn impl_wasm_grammar_data(name: &Ident, data: &Data, attrs: &Vec<Attribute>) -> TokenStream {
    match *data {
        Data::Struct(ref data) => impl_wasm_grammar_struct(name, data, attrs),
        Data::Enum(ref data) => impl_wasm_grammar_enum(name, data, attrs),
        Data::Union(_) => abort!(
            name.span(),
            "Unions cannot be deserialized deriving from the Grammar macro. Consider using an enum instead."
        ),
    }
}