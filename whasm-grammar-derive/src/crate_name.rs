use proc_macro_error::abort;
use proc_macro_crate::crate_name;
use proc_macro2::{ Ident, Span };

pub fn whasm() -> Ident {
    let current_crate = std::env::var("CARGO_PKG_NAME").unwrap_or("".into());
    if current_crate == "whasm" {
        Ident::new("whasm", Span::call_site())
    } else if let Ok(name) = crate_name("whasm") {
        Ident::new(&name, Span::call_site())
    } else {
        abort!(Span::call_site(), "Could not find required crate `whasm`.");
    }
}