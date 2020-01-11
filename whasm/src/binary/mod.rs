//! This module defines the parsing of all the elements required to parse a WebAssembly module.

pub mod core;

pub mod idx;
pub mod instr;
pub mod module;
pub mod ty;

pub use self::core::byte::Byte;
pub use self::core::vec::{CompactVec, UnwrappingVec};
pub use self::core::sized::{Sized, Consume};
pub use self::core::error::Error;
pub use self::core::result::Result;
pub use self::core::traits::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy};