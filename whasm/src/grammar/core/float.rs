//! This module defines the deserialization of floating point numbers (`f32` and `f64`).
//! 
//! The [WebAssembly Specification](https://webassembly.github.io/spec/) specifies that floating
//! point numbers should be serialized using little-endian IEEE 754 encoding.
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0xDB, 0x0F, 0x49, 0x40].iter().copied();
//! let result: f32 = deserialize(&mut iter).unwrap();
//! assert_eq!(result, std::f32::consts::PI);
//! ```

use super::*;
use num_traits::*;

fn deserialize_float<T, Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<T>
where T: Float + Sized {
    let mut result = T::zero();
    let bytes: *mut u8 = (&mut result as *mut T).cast();
    for n in 0..std::mem::size_of::<T>() {
        let Byte(byte) = deserialize(iter)?;
        unsafe { bytes.offset(n as isize).write_unaligned(byte) };
    }
    Ok(result)
}

impl Grammar for f32 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_float(iter)
    }
}
impl Grammar for f64 {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        deserialize_float(iter)
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    #[test]
    fn can_deserialize_f32_pi() {
        let mut iter = [0xDB, 0x0F, 0x49, 0x40].iter().copied();
        let result: f32 = deserialize(&mut iter).unwrap();
        assert_eq!(result, std::f32::consts::PI);
    }

    #[test]
    fn can_deserialize_f64_pi() {
        let mut iter = [0x18, 0x2D, 0x44, 0x54, 0xFB, 0x21, 0x09, 0x40].iter().copied();
        let result: f64 = deserialize(&mut iter).unwrap();
        assert_eq!(result, std::f64::consts::PI);
    }
}