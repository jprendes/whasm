use proc_macro_error::abort;
use proc_macro_crate::crate_name;
use proc_macro2::{ Ident, Span };

pub fn whasm() -> Ident {
    let name = match current_crate() {
        Some(name) if name == "\"whasm\"" => String::from("crate"),
        _ => match crate_name("whasm").ok() {
            Some(name) => name,
            _ => abort!(Span::call_site(), "Could not find required crate `whasm` {:?}.", current_crate()),
        }
    };
    Ident::new(&name, Span::call_site())
}

use toml::{self, value::Table};
use std::io::Read;
type CargoToml = std::collections::HashMap<String, toml::Value>;

fn current_crate() -> Option<String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok()?;
    let cargo_toml_path = std::path::PathBuf::from(manifest_dir).join("Cargo.toml");

    if !cargo_toml_path.exists() { return None; }

    open_cargo_toml(&cargo_toml_path).ok()?
        .remove("package")
        .and_then(|v| v.try_into::<Table>().ok())
        .and_then(|t| t.get("name").cloned())
        .and_then(|n| Some(n.to_string()))
}

fn open_cargo_toml(path: &std::path::Path) -> Result<CargoToml, String> {
    let mut content = String::new();
    std::fs::File::open(path)
        .map_err(|e| format!("Could not open `{}`: {:?}", path.display(), e))?
        .read_to_string(&mut content)
        .map_err(|e| format!("Could not read `{}` to string: {:?}", path.display(), e))?;
    toml::from_str(&content).map_err(|e| format!("{:?}", e))
}