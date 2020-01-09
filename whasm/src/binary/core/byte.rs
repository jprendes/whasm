//! This module defines the parsing of `whasm::binary::Byte`.
//! 
//! A `Byte` is a proxy type for a `u8`. Simple `u8`s are encoded using LEB-128 and may require
//! several bytes to encode, while a `u8` parsed though a `Byte` is interpreted raw from the input
//! iterator and parsing it consumes exactly one byte from the iterator.
//! 
//! # Example
//! 
//! ```
//! # use whasm::binary::{WasmBinary, Byte};
//! let mut iter = [0x8E].iter().copied();
//! let Byte(result) = iter.parse().unwrap();
//! assert_eq!(result, 0x8E);
//! ```

use crate::binary::{WasmBinary, WasmBinaryParseProxy, Result, Error};

/// `Byte` is a proxy for an `u8` representing a byte of information. It differs from an `u8` on
/// how it is parsed. A `Byte` reads exactly one byte from the input iterator, while a `u8` could
/// potentially read several bytes (since unsigned integers are encoded using LEB-128).
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
/// If there are no bytes left in the iterator, parsing returns
/// `Error(Error::UnexpectedEndOfFile)`.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, Result, Byte, Error};
/// let mut iter = [].iter().copied();
/// let result: Result<Byte> = iter.parse();
/// assert_eq!(result, Err(Error::UnexpectedEndOfFile));
/// ```
/// 
/// Parsing a `Byte` instead of using `iter.next()` is a convenient way to convert the `Option<u8>`
/// returned by the iterator into a `Result<>` with a meaningful error type. The parsing of all
/// other structures in this library never read `u8`s directly from the iterator, they parse
/// `Byte`s instead.
pub struct Byte(pub u8);

impl WasmBinaryParseProxy for Byte {
    type Inner = u8;

    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        match bin.next() {
            Some(byte) => Ok(Byte(byte)),
            None => Err(Error::UnexpectedEndOfFile)?,
        }
    }

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