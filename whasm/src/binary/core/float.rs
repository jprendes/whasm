//! This module defines the parsing of floating point numbers.
//! 
//! Floating point numbers are encoded with the little-endian IEEE-754 format.
//! The floating point types are `f32` and `f64`.
//! 
//! # Example
//! 
//! Parsing an `f32` always consumes 4 bytes from the iterator.
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0xDB, 0x0F, 0x49, 0x40].iter().copied();
//! let result: f32 = iter.parse().unwrap();
//! assert_eq!(result, std::f32::consts::PI);
//! assert!(iter.next().is_none());
//! ```
//! 
//! Parsing an `f64` always consumes 8 bytes from the iterator.
//! 
//! ```
//! # use whasm::binary::WasmBinary;
//! let mut iter = [0x18, 0x2D, 0x44, 0x54, 0xFB, 0x21, 0x09, 0x40].iter().copied();
//! let result: f64 = iter.parse().unwrap();
//! assert_eq!(result, std::f64::consts::PI);
//! assert!(iter.next().is_none());
//! ```

use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte};
use num_traits::*;

impl WasmBinaryParse for f32 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_float_iee754(bin)
    }
}

impl WasmBinaryParse for f64 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_float_iee754(bin)
    }
}

fn parse_float_iee754<T, Binary: WasmBinary>(bin: &mut Binary) -> Result<T>
where T: Float + Sized {
    let mut result = T::zero();
    let bytes: *mut u8 = (&mut result as *mut T).cast();
    for n in 0..std::mem::size_of::<T>() {
        let Byte(byte) = bin.parse()?;
        unsafe { bytes.offset(n as isize).write_unaligned(byte) };
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::binary::WasmBinary;

    #[test]
    fn can_parse_f32_pi() {
        let mut iter = [0xDB, 0x0F, 0x49, 0x40].iter().copied();
        let result: f32 = iter.parse().unwrap();
        assert_eq!(result, std::f32::consts::PI);
    }

    #[test]
    fn can_parse_f64_pi() {
        let mut iter = [0x18, 0x2D, 0x44, 0x54, 0xFB, 0x21, 0x09, 0x40].iter().copied();
        let result: f64 = iter.parse().unwrap();
        assert_eq!(result, std::f64::consts::PI);
    }
}