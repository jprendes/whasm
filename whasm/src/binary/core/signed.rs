//! This module defines the parsing of signed integer numbers.
//! 
//! Integer numbers are encoded with the LEB-128 format.
//! The signed integer types are `i8`, `i16`, `i32`, `i64` and `isize`.
//! 
//! # Example
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0x56].iter().copied();
//! let result: i8 = iter.parse().unwrap();
//! assert_eq!(result, -42);
//! ```
//! 
//! The LEB-128 encoding for a number is not unique.
//! A number can be encoded with more bytes than strictly necessary by adding leading zeros or ones
//! (depending on the sign).
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0xD6, 0xFF, 0x7F].iter().copied();
//! let result: i8 = iter.parse().unwrap();
//! assert_eq!(result, -42);
//! ```
//! 
//! If the encoded numeric value is out of range for the resulting type parsing returns
//! `Err(Error::OutOfRangeSignedInteger)`.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, Result, Error};
//! let mut iter = [0xD6, 0x7E].iter().copied();
//! let result: Result<i8> = iter.parse();
//! assert_eq!(result, Err(Error::OutOfRangeSignedInteger));
//! ```

use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, Error};
use num_traits::*;

impl WasmBinaryParse for i8 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

impl WasmBinaryParse for i16 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

impl WasmBinaryParse for i32 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

impl WasmBinaryParse for i64 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

fn parse_signed_leb_128<T, Binary: WasmBinary>(bin: &mut Binary) -> Result<T>
where T: PrimInt + Signed + FromPrimitive {
    let size = 8 * std::mem::size_of::<T>();
    let mask = T::from_u8(0x7F).unwrap();
    let mut result = T::zero();
    let mut shift = 0;
    let mut positive = true;
    loop {
        let Byte(byte) = bin.parse()?;
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

#[cfg(test)]
mod test {
    use crate::binary::{WasmBinary, Result, Error};

    // spec positive examples
    #[test]
    fn can_parse_spec_i8() {
        let mut iter = [0x7E].iter().copied();
        let result: i8 = iter.parse().unwrap();
        assert_eq!(result, -2);
    }

    #[test]
    fn can_parse_spec_multibyte_i8() {
        let mut iter = [0xFE, 0x7F].iter().copied();
        let result: i8 = iter.parse().unwrap();
        assert_eq!(result, -2);
    }

    #[test]
    fn can_parse_second_spec_multibyte_i8() {
        let mut iter = [0xFE, 0xFF, 0x7F].iter().copied();
        let result: i8 = iter.parse().unwrap();
        assert_eq!(result, -2);
    }

    // spec negative examples
    #[test]
    fn fails_to_parse_spec_multibyte_i8() {
        let mut iter = [0x83, 0x3E].iter().copied();
        let result: Result<i8> = iter.parse();
        assert_eq!(result, Err(Error::OutOfRangeSignedInteger));
    }

    #[test]
    fn fails_to_parse_second_spec_multibyte_i8() {
        let mut iter = [0xFF, 0x7B].iter().copied();
        let result: Result<i8> = iter.parse();
        assert_eq!(result, Err(Error::OutOfRangeSignedInteger));
    }

    // i8 tests
    #[test]
    fn can_parse_positive_i8() {
        let mut iter = [0x2A].iter().copied();
        let result: i8 = iter.parse().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn can_parse_multibyte_positive_i8() {
        let mut iter = [0xEA, 0x00].iter().copied();
        let result: i8 = iter.parse().unwrap();
        assert_eq!(result, 106);
    }

    #[test]
    fn fails_to_parse_overflowing_positive_i8() {
        let mut iter = [0xD6, 0x81, 0x80, 0x00].iter().copied();
        let result: Result<i8> = iter.parse();
        assert_eq!(result, Err(Error::OutOfRangeSignedInteger));
    }

    #[test]
    fn fails_to_parse_overflowing_negative_i8() {
        let mut iter = [0xD6, 0xFE, 0x80, 0x7F].iter().copied();
        let result: Result<i8> = iter.parse();
        assert_eq!(result, Err(Error::OutOfRangeSignedInteger));
    }
}