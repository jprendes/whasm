//! This module defines the parsing of vectors.
//! 
//! The following type of vectors are implemented in this module:
//! 
//! # `Vec<T>`
//! 
//! The standard vector type.
//! A `Vec<T>` is encoded specifying the total number `N` of elements as `u32`, followed by `N`
//! encoded elements of type `T`.
//! 
//! # `CompactVec<T>`
//! 
//! A proxy type for `Vec<T>`.
//! A `CompactVec<T>` is encoded specifying the number `N` of tuples as `u32`, followed by `N`
//! tuples.
//! Each tuple consists of the number `K` of repetition as `u32`, followed by an element of type
//! `T`.
//! A tuple means that the vector contains `K` consecutive repetitions of the accompanying value.
//! 
//! # `UnwrappingVec<T>`
//! 
//! A proxy type for `Vec<T::Inner>`, where `T` is itself a proxy type.
//! A `Vec<T>` is encoded specifying the total number `N` of elements as `u32`, followed by `N`
//! encoded elements of type `T`, which are immediately unwrapped into the vector.
//! 
//! # Example
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0x04, 0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: Vec<u8> = iter.parse().unwrap();
//! assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
//! ```
//! 
//! A `CompactVec<_>` provides a more compact representation when may similar values are repeated.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, core::vec::CompactVec};
//! let mut iter = [0x02, 0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: Vec<u8> = iter.parse::<CompactVec<_>>().unwrap().unwrap();
//! assert_eq!(result, [0x02, 0x04, 0x04, 0x04]);
//! ```
//! 
//! An `UnwrappingVec<_>` unwraps its elements before storing them.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, Byte, core::vec::UnwrappingVec};
//! let mut iter = [0x03, 0x2A, 0x8A, 0x42].iter().copied();
//! let result: Vec<u8> = iter.parse::<UnwrappingVec<Byte>>().unwrap().unwrap();
//! assert_eq!(result, vec![0x2A, 0x8A, 0x42]);
//! ```

use crate::binary::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy, Result};

impl<T: WasmBinaryParse> WasmBinaryParse for Vec<T> {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let n: u32 = bin.parse()?;
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            result.push(bin.parse()?);
        }
        Ok(result)
    }
}

pub struct CompactVec<T> ( Vec<T> );
impl<T: WasmBinaryParse + Copy> WasmBinaryParse for CompactVec<T> {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let mut result = vec![];
        let k: u32 = bin.parse()?;
        for _ in 0..k {
            let n: u32 = bin.parse()?;
            let val = bin.parse()?;
            for _ in 0..n {
                result.push(val);
            }
        }
        Ok(Self(result))
    }
}
impl<T: WasmBinaryParse + Copy> WasmBinaryParseProxy for CompactVec<T> {
    type Inner = Vec<T>;
    fn unwrap(self) -> Self::Inner { self.0 }
}

pub struct UnwrappingVec<T: WasmBinaryParseProxy> ( Vec<T::Inner> );
impl<T: WasmBinaryParseProxy> WasmBinaryParse for UnwrappingVec<T> {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let n: u32 = bin.parse()?;
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            result.push(<T as WasmBinaryParse>::parse(bin)?.unwrap());
        }
        Ok(Self(result))
    }
}
impl<T: WasmBinaryParseProxy> WasmBinaryParseProxy for UnwrappingVec<T> {
    type Inner = Vec<T::Inner>;
    fn unwrap(self) -> Self::Inner { self.0 }
}

#[cfg(test)]
mod test {
    use crate::binary::{WasmBinary, WasmBinaryParseProxy, Byte};
    use crate::binary::core::vec::{CompactVec, UnwrappingVec};

    #[test]
    fn can_deserialize_vec() {
        let mut iter = [0x04, 0x01, 0x02, 0x03, 0x04].iter().copied();
        let result: Vec<u8> = iter.parse().unwrap();
        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn can_deserialize_compact_vec() {
        let mut iter = [0x02, 0x02, 0x2A, 0x03, 0x8E, 0x01].iter().copied();
        let result: Vec<u8> = iter.parse::<CompactVec<_>>().unwrap().unwrap();
        assert_eq!(result, vec![42, 42, 142, 142, 142]);
    }

    #[test]
    fn can_deserialize_unwrapping_vec() {
        let mut iter = [0x03, 0x2A, 0x2A, 0x2A].iter().copied();
        let result: Vec<u8> = iter.parse::<UnwrappingVec<Byte>>().unwrap().unwrap();
        assert_eq!(result, vec![42, 42, 42]);
    }
}