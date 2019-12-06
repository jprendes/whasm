use proc_macro_error::abort;
use proc_macro2::{ TokenStream, TokenTree };
use syn::{ Attribute, spanned::Spanned };

pub fn get_attribute(attr_name: &str, attrs: &Vec<Attribute>) -> Option<TokenStream> {
    attrs.iter()
        .filter(|attr| {
            match attr.path.get_ident() {
                Some(ident) => format!("{}", ident) == attr_name,
                None => false
            }
        })
        .last()
        .and_then(|attr| {
            let tree = attr.tokens.clone().into_iter().next();
            match tree {
                Some(TokenTree::Group(ref group)) => Some(group.stream()),
                None => Some(TokenStream::new()),
                _ => abort!(
                    attr.tokens.span(),
                    "Invalid value for attribute `{}`. Use `{}(value)` instead.",
                    stringify!(attr_name), stringify!(attr_name)
                ),
            }
        })
}