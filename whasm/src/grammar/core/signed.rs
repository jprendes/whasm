//! This module defines the deserialization of signed integer numbers
//! (`i8`, `i16`, `i32`, `i64` and `isize`).
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x56].iter().copied();
//! let result: i8 = deserialize(&mut iter).unwrap();
//! assert_eq!(result, -42);
//! ```
//! 
//! The [WebAssembly Specification](https://webassembly.github.io/spec/) specifies that signed
//! integer numbers should be serialized using signed LEB-128 encoding. This means that a number
//! can be encoded with a variable number of bytes.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xD6, 0xFF, 0x7F].iter().copied();
//! let result: i8 = deserialize(&mut iter).unwrap();
//! assert_eq!(result, -42);
//! ```
//! 
//! Deserializing a signed integer will return the `Err(_)` variant if the iterator is exhausted
//! before completing the deserialization.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xD6, 0xFF].iter().copied();
//! let result: Result<i8> = deserialize(&mut iter);
//! assert!(result.is_err());
//! ```
//! 
//! Deserialization will also return the `Err(_)` variant if the encoded numeric value is out of
//! bound for the type being deserialized.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xD6, 0x7E].iter().copied();
//! let result: Result<i8> = deserialize(&mut iter);
//! assert!(result.is_err());
//! ```

use super::*;
use num_traits::*;

fn deserialize_signed_leb_128<T, Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<T>
where T: PrimInt + Signed + FromPrimitive {
    let size = 8 * std::mem::size_of::<T>();
    let mask = T::from_u8(0x7F).unwrap();
    let mut result = T::zero();
    let mut shift = 0;
    let mut positive = true;
    loop {
        let Byte(byte) = deserialize(iter)?;
        let val = T::from_u8(byte & 0x7F).unwrap();
        if shift >= size {
            match byte & 0x7F {
                0x00 if positive => (),
                0x7F if !positive => (),
                _ => Err(Error::OutOfRangeSignedInteger)?,
            }
            if byte & 0x80 == 0 {
                break;
            } else {
                continue;
            }
        }
        if ( ((val << shift) >> shift) & mask ) != val {
            Err(Error::OutOfRangeSignedInteger)?;
        }
        if shift < size {
            result = result | ( val << shift);
            shift += 7;
        }
        positive = (byte & 0x40) == 0;
        if byte & 0x80 == 0 {
            break;
        }
    }
    if ( shift < size ) && !positive {
        result = result | ( (!T::zero()) << shift );
    }
    Ok(result)
}

impl Grammar for i8 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_signed_leb_128(iter)
    }
}
impl Grammar for i16 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_signed_leb_128(iter)
    }
}
impl Grammar for i32 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_signed_leb_128(iter)
    }
}
impl Grammar for i64 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_signed_leb_128(iter)
    }
}
impl Grammar for isize {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_signed_leb_128(iter)
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    // spec positive examples
    #[test]
    fn can_deserialize_spec_i8() {
        let mut iter = [0x7E].iter().copied();
        let result: i8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, -2);
    }

    #[test]
    fn can_deserialize_spec_multibyte_i8() {
        let mut iter = [0xFE, 0x7F].iter().copied();
        let result: i8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, -2);
    }

    #[test]
    fn can_deserialize_second_spec_multibyte_i8() {
        let mut iter = [0xFE, 0xFF, 0x7F].iter().copied();
        let result: i8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, -2);
    }

    // spec negative examples
    #[test]
    #[should_panic]
    fn fails_to_deserialize_spec_multibyte_i8() {
        let mut iter = [0x83, 0x3E].iter().copied();
        let _: i8 = deserialize(&mut iter).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_second_spec_multibyte_i8() {
        let mut iter = [0xFF, 0x7B].iter().copied();
        let _: i8 = deserialize(&mut iter).unwrap();
    }

    // i8 tests
    #[test]
    fn can_deserialize_positive_i8() {
        let mut iter = [0x2A].iter().copied();
        let result: i8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn can_deserialize_multibyte_positive_i8() {
        let mut iter = [0xEA, 0x00].iter().copied();
        let result: i8 = deserialize(&mut iter).unwrap();
        assert_eq!(result, 106);
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_overflowing_positive_i8() {
        let mut iter = [0xD6, 0x81, 0x80, 0x00].iter().copied();
        let _: i8 = deserialize(&mut iter).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_overflowing_negative_i8() {
        let mut iter = [0xD6, 0xFE, 0x80, 0x7F].iter().copied();
        let _: i8 = deserialize(&mut iter).unwrap();
    }
}