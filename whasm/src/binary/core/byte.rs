//! This module implements parsing of bytes.
//! 
//! The `Byte` proxy type wraps a `u8` changing its parsing from LEB-182 encoding to raw bytes.
//! 
//! Parsing a `Byte` rather than calling `next` on the iterator is a convenient way to obtain a
//! `Result<Byte>` with a meaningful error state rather than an Option<u8>.
//! Because of this, it is encouraged to parse `Byte`s from an iterator rather than directly
//! calling `next`.

use crate::binary::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy, Result, Error};

/// `Byte` is a proxy type that wraps a `u8`.
/// It implements the `WasmBinaryParseProxy<Inner=u8>` trait.
/// The `u8` obtained though a `Byte` is a raw byte of information (not an unsigned number).
/// It differs from an unwrapped `u8` on its parsing: a `Byte` reads exactly one byte from the
/// input iterator, while an unwrapped `u8` is encoded using LEB-128 and could potentially read
/// several bytes.
/// 
/// # Example
/// 
/// ```
/// # use whasm::binary::{WasmBinary, Byte};
/// let mut iter = [0x8E].iter().copied();
/// let Byte(result) = iter.parse().unwrap();
/// assert_eq!(result, 0x8E);
/// ```
/// 
/// Calling `unwrap` on a `Byte` returns the contained `u8`.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, Byte};
/// let byte = Byte(42);
/// let result = byte.unwrap();
/// assert_eq!(result, 42);
/// ```
/// 
/// If there are no bytes left in the iterator parsing returns `Error(Error::UnexpectedEndOfFile)`.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, Result, Byte, Error};
/// let mut iter = [].iter().copied();
/// let result: Result<Byte> = iter.parse();
/// assert_eq!(result, Err(Error::UnexpectedEndOfFile));
/// ```
pub struct Byte(pub u8);

impl WasmBinaryParse for Byte {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        match bin.next() {
            Some(byte) => Ok(Byte(byte)),
            None => Err(Error::UnexpectedEndOfFile)?,
        }
    }
}
impl WasmBinaryParseProxy for Byte {
    type Inner = u8;
    fn unwrap(self) -> Self::Inner { self.0 }
}

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

impl std::cmp::PartialEq<u8> for Byte {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialEq<char> for Byte {
    fn eq(&self, other: &char) -> bool {
        (self.0 as char) == *other
    }
}

impl std::cmp::PartialEq<Byte> for Byte {
    fn eq(&self, other: &Byte) -> bool {
        self.0 == other.0
    }
}

impl From<Byte> for u8 {
    fn from(byte: Byte) -> u8 { byte.0 }
}

impl From<Byte> for char {
    fn from(byte: Byte) -> char { byte.0.into() }
}

#[cfg(test)]
mod test {
    use crate::binary::{WasmBinary, WasmBinaryParseProxy, Byte, Result, Error};

    #[test]
    fn can_parse_byte() {
        let mut iter = [0x8E].iter().copied();
        let Byte(result) = iter.parse().unwrap();
        assert_eq!(result, 0x8E);
    }

    #[test]
    fn can_return_eof_error() {
        let mut iter = [].iter().copied();
        let result: Result<Byte> = iter.parse();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile));
    }

    #[test]
    fn can_unwrap_byte() {
        let byte = Byte(42);
        let result = byte.unwrap();
        assert_eq!(result, 42);
    }
}