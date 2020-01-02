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