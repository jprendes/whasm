//! This module defines the parsing of a WebAssembly module.

use crate::binary::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy, Byte, Result, Error};
use crate::binary::{Sized, Consume, UnwrappingVec, CompactVec};
use crate::structure::{module, idx};

impl WasmBinaryParse for (String, Consume) {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok((bin.parse()?, bin.parse()?))
    }
}

impl WasmBinaryParse for module::Module {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let magic: [Byte; 4] = bin.parse()?;
        if magic != ['\0', 'a', 's', 'm'] {
            return Err(Error::InvalidPreambleMagic)
        }
        let version: [Byte; 4] = bin.parse()?;
        if version != [1, 0, 0, 0] {
            return Err(Error::InvalidPreambleVersion)
        }

        let mut func_types = vec![];
        let mut result = module::Module::default();

        let mut bin = bin.peekable();
        let mut last_id = 0;

        while bin.peek().is_some() {
            let Byte(id) = bin.parse()?;

            if id != 0x00 {
                if id <= last_id {
                    // follow strict section order
                    return Err(Error::UnexpectedSectionId{ id: Byte(id) })
                } else {
                    last_id = id;
                }
            }

            match id {
                0x00 => {
                    let (_name, _) : (String, Consume) = bin.parse::<Sized<_>>()?.unwrap();
                },
                0x01 => result.types = bin.parse::<Sized<_>>()?.unwrap(),
                0x02 => result.imports = bin.parse::<Sized<_>>()?.unwrap(),
                0x03 => func_types = bin.parse::<Sized<_>>()?.unwrap(),
                0x04 => result.tables = bin.parse::<Sized<_>>()?.unwrap(),
                0x05 => result.mems = bin.parse::<Sized<_>>()?.unwrap(),
                0x06 => result.globals = bin.parse::<Sized<_>>()?.unwrap(),
                0x07 => result.exports = bin.parse::<Sized<_>>()?.unwrap(),
                0x08 => result.start = Some(bin.parse::<Sized<_>>()?.unwrap()),
                0x09 => result.elem = bin.parse::<Sized<_>>()?.unwrap(),
                0x0A => {
                    result.funcs = bin.parse::<Sized<UnwrappingVec<Sized<_>>>>()?.unwrap().unwrap();
                    let n = func_types.len();
                    if result.funcs.len() != n {
                        return Err(Error::UnexpectedEndOfFile);
                    }
                    for i in 0..n {
                        result.funcs[i].ty = func_types[i];
                    }
                },
                0x0B => result.data = bin.parse::<Sized<_>>()?.unwrap(),
                _ => return Err(Error::InvalidSectionId{ id: Byte(id) })
            }
        }

        Ok(result)
    }
}

impl WasmBinaryParse for module::Func {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let locals = bin.parse::<CompactVec<_>>()?.unwrap();
        let body = bin.parse()?;

        Ok(Self {
            ty: idx::Type(0),
            locals: locals,
            body: body,
        })
    }
}

impl WasmBinaryParse for module::Table {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            ty: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for module::Mem {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            ty: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for module::Global {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            ty: bin.parse()?,
            init: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for module::Elem {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            table: bin.parse()?,
            offset: bin.parse()?,
            init: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for module::Data {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            mem: bin.parse()?,
            offset: bin.parse()?,
            init: bin.parse::<UnwrappingVec<Byte>>()?.unwrap(),
        })
    }
}

impl WasmBinaryParse for module::Start {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            func: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for module::desc::Import {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x00 => Ok(Self::Func(bin.parse()?)),
            0x01 => Ok(Self::Table(bin.parse()?)),
            0x02 => Ok(Self::Mem(bin.parse()?)),
            0x03 => Ok(Self::Global(bin.parse()?)),
            id => Err(Error::InvalidVariantId {id, ty: "module::desc::Import"}),
        }
    }
}

impl WasmBinaryParse for module::desc::Export {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            0x00 => Ok(Self::Func(bin.parse()?)),
            0x01 => Ok(Self::Table(bin.parse()?)),
            0x02 => Ok(Self::Mem(bin.parse()?)),
            0x03 => Ok(Self::Global(bin.parse()?)),
            id => Err(Error::InvalidVariantId {id, ty: "module::desc::Export"}),
        }
    }
}

impl WasmBinaryParse for module::Import {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            module: bin.parse()?,
            name: bin.parse()?,
            desc: bin.parse()?,
        })
    }
}

impl WasmBinaryParse for module::Export {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Self {
            name: bin.parse()?,
            desc: bin.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::structure::{module, ty, idx, instr};
    use crate::binary::WasmBinary;

    #[test]
    fn can_deserialize_wasm_file() {
        let mut iter = [
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x85, 0x80, 0x80, 0x80, 0x00, 0x01, 0x60,
            0x00, 0x01, 0x7f, 0x03, 0x82, 0x80, 0x80, 0x80, 0x00, 0x01, 0x00, 0x04, 0x84, 0x80, 0x80, 0x80,
            0x00, 0x01, 0x70, 0x00, 0x00, 0x06, 0x81, 0x80, 0x80, 0x80, 0x00, 0x00, 0x07, 0x88, 0x80, 0x80,
            0x80, 0x00, 0x01, 0x04, 0x74, 0x65, 0x73, 0x74, 0x00, 0x00, 0x0a, 0x8a, 0x80, 0x80, 0x80, 0x00,
            0x01, 0x84, 0x80, 0x80, 0x80, 0x00, 0x00, 0x41, 0x2a, 0x0b,
        ].iter().copied();
        let result: module::Module = iter.parse().unwrap();

        assert_eq!(result, module::Module {
            types: vec![
                ty::Func { params: vec![], results: vec![ty::Val::I32] },
            ],
            funcs: vec![
                module::Func {
                    ty: idx::Type(0),
                    locals: vec![],
                    body: instr::Expr(vec![
                        instr::Instr::ConstI32(42)
                    ])
                },
            ],
            tables: vec![
                module::Table {
                    ty: ty::Table {
                        lim: ty::Limits { min: 0, max: None },
                        elem: ty::Elem::FuncRef
                    }
                },
            ],
            mems: vec![],
            globals: vec![],
            elem: vec![],
            data: vec![],
            start: None,
            imports: vec![],
            exports: vec![
                module::Export {
                    name: "test".into(),
                    desc: module::desc::Export::Func( idx::Func(0) )
                },
            ]
        });
    }
}
