use crate::binary::{WasmBinary, WasmBinaryParse, Result};

pub trait WasmBinaryParseProxy : WasmBinaryParse {
    type Inner;
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self>;
    fn unwrap(self) -> Self::Inner;
}

impl<T: WasmBinaryParseProxy> WasmBinaryParse for T {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        <Self as WasmBinaryParseProxy>::parse(bin)
    }
}