//! This module defines the the `whasm::binary::WasmBinaryParseProxy` trait.
//! 
//! The `WasmBinaryParseProxy` trait is used for types intended to wrap another type. This allows
//! the proxy type to influence the parsing of the wrapped type.
//! 
//! Proxy types implement the `unwrap` method, that returns the wrapped type.
//! 
//! # Example
//! 
//! The `whasm::binary::Byte` is a proxy for an `u8`. While an `u8` is normally parsed using the
//! LEB-128 encding, an `u8` wrapped by a Byte is obtained as a raw byte from the iterator.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, Byte};
//! let mut iter = [0x8E].iter().copied();
//! let result: Byte = iter.parse().unwrap();
//! assert_eq!(result.unwrap(), 0x8E);
//! ```
//! 
//! A proxy that discards the first byte before the encoded element.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy, Result, Byte};
//! struct SkipOneByte<T> ( T );
//! 
//! impl<T: WasmBinaryParse> WasmBinaryParseProxy for SkipOneByte<T> {
//!     type Inner = T;
//!     fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
//!         let _: Byte = bin.parse()?;
//!         Ok(Self(bin.parse()?))
//!     }
//!     fn unwrap(self) -> Self::Inner {
//!         self.0
//!     }
//! }
//! 
//! let mut iter = [0x00, 0x2A].iter().copied();
//! let result: SkipOneByte<Byte> = iter.parse().unwrap();
//! assert_eq!(result.unwrap(), 42)
//! ```

use crate::binary::{WasmBinary, WasmBinaryParse, Result};

pub trait WasmBinaryParseProxy : WasmBinaryParse {
    type Inner;
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self>;
    fn unwrap(self) -> Self::Inner;
}

impl<T: WasmBinaryParseProxy> WasmBinaryParse for T {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        <Self as WasmBinaryParseProxy>::parse(bin)
    }
}