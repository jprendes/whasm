//! This module defines a trait for proxy types: types that wrap other types to change their
//! parsing rules.
//! 
//! Proxy types implement the `whasm::binary::WasmBinaryParseProxy` trait, which is defined in this
//! module.
//! Proxy types are intended to wrap another type to change the parsing rules of the wrapped type.
//! 
//! Proxy types must implement the `WasmBinaryParse` trait as well as the `unwrap` methods of
//! `WasmBinaryParseProxy`.
//! The `unwrap` method returns the wrapped type.
//! 
//! # Example
//! 
//! The type `whasm::binary::Byte` is a proxy for an `u8`. It changes the parsing of an `u8` from
//! LEB-128 encoding, to parsing a raw byte from an iterator.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, Byte};
//! let mut iter = [0x8E].iter().copied();
//! let result = iter.parse::<Byte>().unwrap().unwrap();
//! assert_eq!(result, 0x8E);
//! 
//! let mut iter = [0x8E, 0x00].iter().copied();
//! let result = iter.parse::<u8>().unwrap();
//! assert_eq!(result, 0x0E);
//! ```
//! 
//! A proxy that discards the first byte before the encoded element.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy, Result, Byte};
//! struct SkipOneByte<T> ( T );
//! impl<T: WasmBinaryParse> WasmBinaryParse for SkipOneByte<T> {
//!     fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
//!         let _: Byte = bin.parse()?;
//!         Ok(Self(bin.parse()?))
//!     }
//! }
//! impl<T: WasmBinaryParse> WasmBinaryParseProxy for SkipOneByte<T> {
//!     type Inner = T;
//!     fn unwrap(self) -> Self::Inner {
//!         self.0
//!     }
//! }
//! 
//! let mut iter = [0x00, 0x2A].iter().copied();
//! let result: SkipOneByte<Byte> = iter.parse().unwrap();
//! assert_eq!(result.unwrap(), 42)
//! ```

use crate::binary::{WasmBinaryParse};

pub trait WasmBinaryParseProxy : WasmBinaryParse {
    type Inner;
    fn unwrap(self) -> Self::Inner;
}