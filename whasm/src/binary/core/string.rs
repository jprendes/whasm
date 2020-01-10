//! This module defines the parsing of strings.
//! 
//! Strings are encoded as `Vec<Byte>` using UTF-8 encoding.
//! 
//! # Example
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0x11, 0x48, 0x65, 0x6C, 0x6C,
//!     0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64,
//!     0x21, 0x20, 0xF0, 0x9F, 0x92, 0x96].iter().copied();
//! let result: String = iter.parse().unwrap();
//! assert_eq!(result, "Hello World! 💖");
//! ```

use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, core::proxy::WasmBinaryParseProxy, core::vec::UnwrappingVec};

impl WasmBinaryParse for String {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let raw: UnwrappingVec<Byte> = bin.parse()?;
        Ok(String::from_utf8(raw.unwrap())?)
    }
}