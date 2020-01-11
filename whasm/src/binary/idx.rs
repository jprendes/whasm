//! This module defines the parsing of the different indexes found WebAssembly.

use crate::binary::{WasmBinaryParse, WasmBinary, Result};
use crate::structure::idx;

impl WasmBinaryParse for idx::Type {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}

impl WasmBinaryParse for idx::Func {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}

impl WasmBinaryParse for idx::Table {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}

impl WasmBinaryParse for idx::Mem {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}

impl WasmBinaryParse for idx::Global {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}

impl WasmBinaryParse for idx::Local {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}

impl WasmBinaryParse for idx::Label {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self(bin.parse()?))
    }
}