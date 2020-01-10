//! This module defines the parsing of unsigned integer numbers.
//! 
//! Integer numbers are encoded with the LEB-128 format.
//! The unsigned integer types are `i8`, `i16`, `i32`, `i64` and `isize`.
//! 
//! # Example
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0x2A].iter().copied();
//! let result: u8 = iter.parse().unwrap();
//! assert_eq!(result, 42);
//! ```
//! 
//! The LEB-128 encoding for a number is not unique.
//! A number can be encoded with more bytes than strictly necessary by adding leading zeros.
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0xAA, 0x80, 0x00].iter().copied();
//! let result: u8 = iter.parse().unwrap();
//! assert_eq!(result, 42);
//! ```
//! 
//! If the encoded numeric value is out of range for the resulting type parsing returns
//! `Err(Error::OutOfRangeUnsignedInteger)`.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, Result, Error};
//! let mut iter = [0xAA, 0x02].iter().copied();
//! let result: Result<u8> = iter.parse();
//! assert_eq!(result, Err(Error::OutOfRangeUnsignedInteger));
//! ```

use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, Error};
use num_traits::*;

impl WasmBinaryParse for u8 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

impl WasmBinaryParse for u16 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

impl WasmBinaryParse for u32 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

impl WasmBinaryParse for u64 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

fn parse_unsigned_leb_128<T, Binary: WasmBinary>(bin: &mut Binary) -> Result<T>
where T: PrimInt + Unsigned + FromPrimitive {
    let size = 8 * std::mem::size_of::<T>();
    let mask = T::from_u8(0x7F).unwrap();
    let mut result = T::zero();
    let mut shift = 0;
    loop {
        let Byte(byte) = bin.parse()?;
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

#[cfg(test)]
mod test {
    use crate::binary::{WasmBinary, Result, Error};

    // spec positive examples
    #[test]
    fn can_parse_spec_u8() {
        let mut iter = [0x03].iter().copied();
        let result: u8 = iter.parse().unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn can_parse_multibyte_spec_u8() {
        let mut iter = [0x83, 0x00].iter().copied();
        let result: u8 = iter.parse().unwrap();
        assert_eq!(result, 3);
    }

    // spec negative examples
    #[test]
    fn fails_to_parse_spec_multibyte_u8() {
        let mut iter = [0x83, 0x10].iter().copied();
        let result: Result<u8> = iter.parse();
        assert_eq!(result, Err(Error::OutOfRangeUnsignedInteger));
    }

    // u8 tests
    #[test]
    fn can_parse_multibyte_u8() {
        let mut iter = [0x8E, 0x81, 0x80, 0x00].iter().copied();
        let result: u8 = iter.parse().unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn fails_to_parse_overflowing_u8() {
        let mut iter = [0x8E, 0x82, 0x80, 0x00].iter().copied();
        let result: Result<u8> = iter.parse();
        assert_eq!(result, Err(Error::OutOfRangeUnsignedInteger));
    }
}