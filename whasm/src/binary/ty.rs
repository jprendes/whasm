use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, Error};
use crate::structure::ty;

impl WasmBinaryParse for ty::Limits {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x00 => Ok(Self {
                min: bin.parse()?,
                max: None,
            }),
            0x01 => Ok(Self {
                min: bin.parse()?,
                max: Some(bin.parse()?),
            }),
            id => Err(Error::InvalidVariantId{id, ty: "ty::Limits"})
        }
    }
}

impl WasmBinaryParse for ty::Val {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            id => Err(Error::InvalidVariantId{id, ty: "ty::Val"})
        }
    }
}

impl WasmBinaryParse for ty::Elem {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x70 => Ok(Self::FuncRef),
            id => Err(Error::InvalidVariantId{id, ty: "ty::Elem"})
        }
    }
}

impl WasmBinaryParse for ty::RetVal {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x40 => Ok(Self(vec![])),
            _ => {
                let byte = [byte];
                let mut iter = byte.iter().copied();
                Ok(Self(vec![iter.parse()?]))
            }
        }
    }
}

impl WasmBinaryParse for ty::Func {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x60 => Ok(Self {
                params: bin.parse()?,
                results: bin.parse()?,
            }),
            id => Err(Error::InvalidVariantId{id, ty: "ty::Func"})
        }
    }
}

impl WasmBinaryParse for ty::Table {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            elem: bin.parse()?,
            lim: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for ty::Mem {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            lim: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for ty::Mut {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x00 => Ok(Self::Const),
            0x01 => Ok(Self::Var),
            id => Err(Error::InvalidVariantId{id, ty: "ty::Mut"})
        }
    }
}

impl WasmBinaryParse for ty::Global {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            val: bin.parse()?,
            mt: bin.parse()?,
        })
    }
}