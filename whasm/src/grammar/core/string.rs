//! This module defines the deserialization of `String`s.
//! 
//! The [WebAssembly Specification](https://webassembly.github.io/spec/) specifies that `String`s
//! are serialized as a `Vec<Byte>` using UTF-8 encoding.
//! 
//! # Example
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x11, 0x48, 0x65, 0x6C, 0x6C,
//!     0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64,
//!     0x21, 0x20, 0xF0, 0x9F, 0x92, 0x96].iter().copied();
//! let result: String = deserialize(&mut iter).unwrap();
//! assert_eq!(result, "Hello World! 💖");
//! ```

use super::*;

impl Grammar for String {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        let raw: Vec<Byte> = deserialize(iter)?;
        let raw: Vec<u8> = unsafe { std::mem::transmute(raw) };
        Ok(String::from_utf8(raw)?)
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    #[test]
    fn can_deserialize_ascii_string() {
        let mut iter = [0x0C, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57,
            0x6F, 0x72, 0x6C, 0x64, 0x21].iter().copied();
        let result: String = deserialize(&mut iter).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn can_deserialize_multibyte_utf8_string() {
        let mut iter = [0x0A, 0x48, 0x65, 0x61, 0x72, 0x74, 0x20, 0xF0,
            0x9F, 0x92, 0x96].iter().copied();
        let result: String = deserialize(&mut iter).unwrap();
        assert_eq!(result, "Heart 💖");
    }
}