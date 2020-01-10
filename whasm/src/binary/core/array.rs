//! This module defines the deserialization of arrays of up to 8 elements.
//! 
//! An array of size `N` is encoded as `N` consecutive elements of the same type.
//! An encoded array differs from an encoded `Vec<_>` in that the array does not encode its size.
//! 
//! # Example
//! 
//! ```
//! # use whasm::binary::{WasmBinary, Byte};
//! let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: [Byte; 4] = iter.parse().unwrap();
//! assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
//! ```

use crate::binary::{WasmBinaryParse, WasmBinary, Result};

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 0] {
    fn parse<Binary: WasmBinary>(_: &mut Binary) -> Result<Self> {
        Ok([])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 1] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 2] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 3] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 4] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 5] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 6] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 7] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

impl<T: WasmBinaryParse> WasmBinaryParse for [T; 8] {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok([
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
            bin.parse()?,
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::binary::{WasmBinary, Byte};

    #[test]
    fn can_deserialize_empty_array() {
        let mut iter = [0; 0].iter().copied();
        let result: [Byte; 0] = iter.parse().unwrap();
        assert_eq!(result, [0; 0]);
    }

    #[test]
    fn can_deserialize_4_element_array() {
        let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
        let result: [Byte; 4] = iter.parse().unwrap();
        assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
    }
}