use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, core::proxy::WasmBinaryParseProxy, core::vec::UnwrappingVec};

impl WasmBinaryParse for String {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let raw: UnwrappingVec<Byte> = bin.parse()?;
        Ok(String::from_utf8(raw.unwrap())?)
    }
}