//! This module defines the deserialization of unsigned integer numbers
//! (`u8`, `u16`, `u32`, `u64` and `usize`).
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x2A].iter().copied();
//! let result: u8 = deserialize(&mut iter).unwrap();
//! assert_eq!(result, 42);
//! ```
//! 
//! The [WebAssembly Specification](https://webassembly.github.io/spec/) specifies that signed
//! integer numbers should be serialized using signed LEB-128 encoding. This means that a number
//! can be encoded with a variable number of bytes.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xAA, 0x80, 0x00].iter().copied();
//! let result: u8 = deserialize(&mut iter).unwrap();
//! assert_eq!(result, 42);
//! ```
//! 
//! Deserializing a signed integer will return the `Err(_)` variant if the iterator is exhausted
//! before completing the deserialization.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xAA, 0x80].iter().copied();
//! let result: Result<u8> = deserialize(&mut iter);
//! assert!(result.is_err());
//! ```
//! 
//! Deserialization will also return the `Err(_)` variant if the encoded numeric value is out of
//! bound for the type being deserialized.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xAA, 0x02].iter().copied();
//! let result: Result<u8> = deserialize(&mut iter);
//! assert!(result.is_err());
//! ```

use super::*;
use num_traits::*;

fn deserialize_unsigned_leb_128<T, Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<T>
where T: PrimInt + Unsigned + FromPrimitive {
    let size = 8 * std::mem::size_of::<T>();
    let mask = T::from_u8(0x7F).unwrap();
    let mut result = T::zero();
    let mut shift = 0;
    loop {
        let Byte(byte) = deserialize(iter)?;
        let val = T::from_u8(byte & 0x7F).unwrap();
        if shift >= size {
            if byte & 0x7F != 0 {
                Err(Error::OutOfRangeUnsignedInteger)?;
            }
            if byte & 0x80 == 0 {
                break;
            } else {
                continue;
            }
        }
        if ( ((val << shift) >> shift) & mask ) != val {
            Err(Error::OutOfRangeUnsignedInteger)?;
        }
        if shift < size {
            result = result | ( val << shift);
            shift += 7;
        }
        if byte & 0x80 == 0 {
            break;
        }
    }
    Ok(result)
}

impl Grammar for u8 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_unsigned_leb_128(iter)
    }
}
impl Grammar for u16 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_unsigned_leb_128(iter)
    }
}
impl Grammar for u32 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_unsigned_leb_128(iter)
    }
}
impl Grammar for u64 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_unsigned_leb_128(iter)
    }
}
impl Grammar for usize {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_unsigned_leb_128(iter)
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    // spec positive examples
    #[test]
    fn can_deserialize_spec_u8() {
        let mut iter = [0x03].iter().copied();
        let result: u8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn can_deserialize_multibyte_spec_u8() {
        let mut iter = [0x83, 0x00].iter().copied();
        let result: u8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, 3);
    }

    // spec negative examples
    #[test]
    #[should_panic]
    fn fails_to_deserialize_spec_multibyte_u8() {
        let mut iter = [0x83, 0x10].iter().copied();
        let _: u8 = deserialize(&mut iter).unwrap();
    }

    // u8 tests
    #[test]
    fn can_deserialize_multibyte_u8() {
        let mut iter = [0x8E, 0x81, 0x80, 0x00].iter().copied();
        let result: u8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_overflowing_u8() {
        let mut iter = [0x8E, 0x82, 0x80, 0x00].iter().copied();
        let _: u8 = deserialize(&mut iter).unwrap();
    }
}