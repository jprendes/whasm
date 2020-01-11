//! This module defines the parsing of instructions and expressions found in WebAssembly.

use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, Error};
use crate::structure::{ty, instr, instr::Instr};

impl WasmBinaryParse for instr::Expr {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        match bin.parse()? {
            SubExpr(expr, Instr::End) => Ok(expr),
            SubExpr(_, Instr::Else) => Err(Error::UnexpectedOpcode{ instr: "Else" }),
            SubExpr(_, _) => Err(Error::UnexpectedOpcode{ instr: "Unknown" })
        }
    }
}

impl WasmBinaryParse for instr::ConstExpr {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let instr::Expr(expr) = bin.parse()?;
        Ok(instr::ConstExpr(expr))
    }
}

impl WasmBinaryParse for instr::Instr {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let Byte(byte) = bin.parse()?;
        match byte {
            // Control flow
            0x00 => Ok(Instr::Unreachable),
            0x01 => Ok(Instr::Nop),
            0x02 => Ok(Instr::Block(bin.parse()?, bin.parse()?)),
            0x03 => Ok(Instr::Loop(bin.parse()?, bin.parse()?)),
            0x04 => Ok(bin.parse::<IfInstr>()?.into()),
            0x05 => Ok(Instr::Else),
                // ... reserved ...
            0x0B => Ok(Instr::End),
            0x0C => Ok(Instr::Br(bin.parse()?)),
            0x0D => Ok(Instr::BrIf(bin.parse()?)),
            0x0E => Ok(Instr::BrTable(bin.parse()?, bin.parse()?)),
            0x0F => Ok(Instr::Return),
            0x10 => Ok(Instr::Call(bin.parse()?)),
            0x11 => Ok(Instr::CallIndirect(bin.parse()?, bin.parse()?)),
                // ... reserved ...
            // Parametric
            0x1A => Ok(Instr::Drop),
            0x1B => Ok(Instr::Select),
                // ... reserved ...
            // Variable
            0x20 => Ok(Instr::LocalGet(bin.parse()?)),
            0x21 => Ok(Instr::LocalSet(bin.parse()?)),
            0x22 => Ok(Instr::LocalTee(bin.parse()?)),
            0x23 => Ok(Instr::GlobalGet(bin.parse()?)),
            0x24 => Ok(Instr::GlobalSet(bin.parse()?)),
                // ... reserved ...
            // Memory load
            0x28 => Ok(Instr::I32Load(bin.parse()?, bin.parse()?)),
            0x29 => Ok(Instr::I64Load(bin.parse()?, bin.parse()?)),
            0x2A => Ok(Instr::F32Load(bin.parse()?, bin.parse()?)),
            0x2B => Ok(Instr::F64Load(bin.parse()?, bin.parse()?)),
            0x2C => Ok(Instr::I32Load8S(bin.parse()?, bin.parse()?)),
            0x2D => Ok(Instr::I32Load8U(bin.parse()?, bin.parse()?)),
            0x2E => Ok(Instr::I32Load16S(bin.parse()?, bin.parse()?)),
            0x2F => Ok(Instr::I32Load16U(bin.parse()?, bin.parse()?)),
            0x30 => Ok(Instr::I64Load8S(bin.parse()?, bin.parse()?)),
            0x31 => Ok(Instr::I64Load8U(bin.parse()?, bin.parse()?)),
            0x32 => Ok(Instr::I64Load16S(bin.parse()?, bin.parse()?)),
            0x33 => Ok(Instr::I64Load16U(bin.parse()?, bin.parse()?)),
            0x34 => Ok(Instr::I64Load32S(bin.parse()?, bin.parse()?)),
            0x35 => Ok(Instr::I64Load32U(bin.parse()?, bin.parse()?)),
            // Memory store
            0x36 => Ok(Instr::I32Store(bin.parse()?, bin.parse()?)),
            0x37 => Ok(Instr::I64Store(bin.parse()?, bin.parse()?)),
            0x38 => Ok(Instr::F32Store(bin.parse()?, bin.parse()?)),
            0x39 => Ok(Instr::F64Store(bin.parse()?, bin.parse()?)),
            0x3A => Ok(Instr::I32Store8(bin.parse()?, bin.parse()?)),
            0x3B => Ok(Instr::I32Store16(bin.parse()?, bin.parse()?)),
            0x3C => Ok(Instr::I64Store8(bin.parse()?, bin.parse()?)),
            0x3D => Ok(Instr::I64Store16(bin.parse()?, bin.parse()?)),
            0x3E => Ok(Instr::I64Store32(bin.parse()?, bin.parse()?)),
            // Memory management
            0x3F => Ok(Instr::MemSize(bin.parse()?)),
            0x40 => Ok(Instr::MemGrow(bin.parse()?)),
            // Numeric
            0x41 => Ok(Instr::ConstI32(bin.parse()?)),
            0x42 => Ok(Instr::ConstI64(bin.parse()?)),
            0x43 => Ok(Instr::ConstF32(bin.parse()?)),
            0x44 => Ok(Instr::ConstF64(bin.parse()?)),
            // Numeric without immediate -- i32
            0x45 => Ok(Instr::I32Eqz),
            0x46 => Ok(Instr::I32Eq),
            0x47 => Ok(Instr::I32Ne),
            0x48 => Ok(Instr::I32LtS),
            0x49 => Ok(Instr::I32LtU),
            0x4A => Ok(Instr::I32GtS),
            0x4B => Ok(Instr::I32GtU),
            0x4C => Ok(Instr::I32LeS),
            0x4D => Ok(Instr::I32LeU),
            0x4E => Ok(Instr::I32GeS),
            0x4F => Ok(Instr::I32GeU),
            // Numeric without immediate -- i64
            0x50 => Ok(Instr::I64Eqz),
            0x51 => Ok(Instr::I64Eq),
            0x52 => Ok(Instr::I64Ne),
            0x53 => Ok(Instr::I64LtS),
            0x54 => Ok(Instr::I64LtU),
            0x55 => Ok(Instr::I64GtS),
            0x56 => Ok(Instr::I64GtU),
            0x57 => Ok(Instr::I64LeS),
            0x58 => Ok(Instr::I64LeU),
            0x59 => Ok(Instr::I64GeS),
            0x5A => Ok(Instr::I64GeU),
            // Numeric without immediate -- f32
            0x5B => Ok(Instr::F32Eq),
            0x5C => Ok(Instr::F32Ne),
            0x5D => Ok(Instr::F32Lt),
            0x5E => Ok(Instr::F32Gt),
            0x5F => Ok(Instr::F32Le),
            0x60 => Ok(Instr::F32Ge),
            // Numeric without immediate -- f64
            0x61 => Ok(Instr::F64Eq),
            0x62 => Ok(Instr::F64Ne),
            0x63 => Ok(Instr::F64Lt),
            0x64 => Ok(Instr::F64Gt),
            0x65 => Ok(Instr::F64Le),
            0x66 => Ok(Instr::F64Ge),
            // Numeric without immediate -- i32 cont.
            0x67 => Ok(Instr::I32Clz),
            0x68 => Ok(Instr::I32Ctz),
            0x69 => Ok(Instr::I32Popcnt),
            0x6A => Ok(Instr::I32Add),
            0x6B => Ok(Instr::I32Sub),
            0x6C => Ok(Instr::I32Mul),
            0x6D => Ok(Instr::I32DivS),
            0x6E => Ok(Instr::I32DivU),
            0x6F => Ok(Instr::I32RemS),
            0x70 => Ok(Instr::I32RemU),
            0x71 => Ok(Instr::I32And),
            0x72 => Ok(Instr::I32Or),
            0x73 => Ok(Instr::I32Xor),
            0x74 => Ok(Instr::I32Shl),
            0x75 => Ok(Instr::I32ShrS),
            0x76 => Ok(Instr::I32ShrU),
            0x77 => Ok(Instr::I32Rotl),
            0x78 => Ok(Instr::I32Rotr),
            // Numeric without immediate -- i64 cont.
            0x79 => Ok(Instr::I64Clz),
            0x7A => Ok(Instr::I64Ctz),
            0x7B => Ok(Instr::I64Popcnt),
            0x7C => Ok(Instr::I64Add),
            0x7D => Ok(Instr::I64Sub),
            0x7E => Ok(Instr::I64Mul),
            0x7F => Ok(Instr::I64DivS),
            0x80 => Ok(Instr::I64DivU),
            0x81 => Ok(Instr::I64RemS),
            0x82 => Ok(Instr::I64RemU),
            0x83 => Ok(Instr::I64And),
            0x84 => Ok(Instr::I64Or),
            0x85 => Ok(Instr::I64Xor),
            0x86 => Ok(Instr::I64Shl),
            0x87 => Ok(Instr::I64ShrS),
            0x88 => Ok(Instr::I64ShrU),
            0x89 => Ok(Instr::I64Rotl),
            0x8A => Ok(Instr::I64Rotr),
            // Numeric without immediate -- f32 cont.
            0x8B => Ok(Instr::F32Abs),
            0x8C => Ok(Instr::F32Neg),
            0x8D => Ok(Instr::F32Ceil),
            0x8E => Ok(Instr::F32Floor),
            0x8F => Ok(Instr::F32Trunc),
            0x90 => Ok(Instr::F32Nearest),
            0x91 => Ok(Instr::F32Sqrt),
            0x92 => Ok(Instr::F32Add),
            0x93 => Ok(Instr::F32Sub),
            0x94 => Ok(Instr::F32Mul),
            0x95 => Ok(Instr::F32Div),
            0x96 => Ok(Instr::F32Min),
            0x97 => Ok(Instr::F32Max),
            0x98 => Ok(Instr::F32Copysign),
            // Numeric without immediate -- f64 cont.
            0x99 => Ok(Instr::F64Abs),
            0x9A => Ok(Instr::F64Neg),
            0x9B => Ok(Instr::F64Ceil),
            0x9C => Ok(Instr::F64Floor),
            0x9D => Ok(Instr::F64Trunc),
            0x9E => Ok(Instr::F64Nearest),
            0x9F => Ok(Instr::F64Sqrt),
            0xA0 => Ok(Instr::F64Add),
            0xA1 => Ok(Instr::F64Sub),
            0xA2 => Ok(Instr::F64Mul),
            0xA3 => Ok(Instr::F64Div),
            0xA4 => Ok(Instr::F64Min),
            0xA5 => Ok(Instr::F64Max),
            0xA6 => Ok(Instr::F64Copysign),
            // Numeric without immediate -- conversion
            0xA7 => Ok(Instr::I32WrapI64),
            0xA8 => Ok(Instr::I32TruncF32S),
            0xA9 => Ok(Instr::I32TruncF32U),
            0xAA => Ok(Instr::I32TruncF64S),
            0xAB => Ok(Instr::I32TruncF64U),
            0xAC => Ok(Instr::I64ExtendI32S),
            0xAD => Ok(Instr::I64ExtendI32U),
            0xAE => Ok(Instr::I64TruncF32S),
            0xAF => Ok(Instr::I64TruncF32U),
            0xB0 => Ok(Instr::I64TruncF64S),
            0xB1 => Ok(Instr::I64TruncF64U),
            0xB2 => Ok(Instr::F32ConvertI32S),
            0xB3 => Ok(Instr::F32ConvertI32U),
            0xB4 => Ok(Instr::F32ConvertI64S),
            0xB5 => Ok(Instr::F32ConvertI64U),
            0xB6 => Ok(Instr::F32DemoteF64),
            0xB7 => Ok(Instr::F64ConvertI32S),
            0xB8 => Ok(Instr::F64ConvertI32U),
            0xB9 => Ok(Instr::F64ConvertI64S),
            0xBA => Ok(Instr::F64ConvertI64U),
            0xBB => Ok(Instr::F64PromoteF32),
            0xBC => Ok(Instr::I32ReinterpretF32),
            0xBD => Ok(Instr::I64ReinterpretF64),
            0xBE => Ok(Instr::F32ReinterpretI32),
            0xBF => Ok(Instr::F64ReinterpretI64),
                // ... reserved ...
            id => Err(Error::InvalidVariantId{id, ty: "instr::Instr"})
        }
    }
}

struct SubExpr ( instr::Expr, instr::Instr );

impl WasmBinaryParse for SubExpr {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let mut expr = instr::Expr(vec![]);
        let mut next;
        loop {
            next = bin.parse()?;
            match next {
                Instr::End => break,
                Instr::Else => break,
                _ => expr.0.push(next),
            }
        }
        Ok(Self(expr, next))
    }
}

struct IfInstr (ty::RetVal, instr::Expr, instr::Expr);

impl WasmBinaryParse for IfInstr {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let ret_val = bin.parse()?;
        let mut next;

        let branch_1 = {
            let SubExpr(b,n) = bin.parse()?;
            next = n;
            b
        };

        let branch_2 = if next == Instr::Else {
            let SubExpr(b,n) = bin.parse()?;
            next = n;
            b
        } else {
            instr::Expr(vec![])
        };

        if next == Instr::End {
            Ok(Self(ret_val, branch_1, branch_2))
        } else {
            Err(Error::UnexpectedOpcode{ instr: "Unknown" })
        }
    }
}

impl From<IfInstr> for Instr {
    fn from(if_instr: IfInstr) -> Instr { Instr::If(if_instr.0, if_instr.1, if_instr.2) }
}