use super::{ty, idx};

#[derive(Debug, PartialEq)]
pub struct Expr(pub Vec<Instr>);

#[derive(Debug, PartialEq)]
pub enum Instr {
    // Control flow
    /* 0x00 */ Unreachable,
    /* 0x01 */ Nop,
    /* 0x02 */ Block(ty::RetVal, Expr),
    /* 0x03 */ Loop(ty::RetVal, Expr),
    /* 0x04 */ If(ty::RetVal, Expr, Expr),
    /* 0x05 */ Else,
        // ... reserved ...
    /* 0x0B */ End,
    /* 0x0C */ Br(idx::Label),
    /* 0x0D */ BrIf(idx::Label),
    /* 0x0E */ BrTable(Vec<idx::Label>, idx::Label),
    /* 0x0F */ Return,
    /* 0x10 */ Call(idx::Func),
    /* 0x11 */ CallIndirect(idx::Func, idx::Table),
        // ... reserved ...
    // Parametric
    /* 0x1A */ Drop,
    /* 0x1B */ Select,
        // ... reserved ...
    // Variable
    /* 0x20 */ LocalGet(idx::Local),
    /* 0x21 */ LocalSet(idx::Local),
    /* 0x22 */ LocalTee(idx::Local),
    /* 0x23 */ GlobalGet(idx::Global),
    /* 0x24 */ GlobalSet(idx::Global),
        // ... reserved ...
    // Memory load
    /* 0x28 */ I32Load(u32, u32),
    /* 0x29 */ I64Load(u32, u32),
    /* 0x2A */ F32Load(u32, u32),
    /* 0x2B */ F64Load(u32, u32),
    /* 0x2C */ I32Load8S(u32, u32),
    /* 0x2D */ I32Load8U(u32, u32),
    /* 0x2E */ I32Load16S(u32, u32),
    /* 0x2F */ I32Load16U(u32, u32),
    /* 0x30 */ I64Load8S(u32, u32),
    /* 0x31 */ I64Load8U(u32, u32),
    /* 0x32 */ I64Load16S(u32, u32),
    /* 0x33 */ I64Load16U(u32, u32),
    /* 0x34 */ I64Load32S(u32, u32),
    /* 0x35 */ I64Load32U(u32, u32),
    // Memory store
    /* 0x36 */ I32Store(u32, u32),
    /* 0x37 */ I64Store(u32, u32),
    /* 0x38 */ F32Store(u32, u32),
    /* 0x39 */ F64Store(u32, u32),
    /* 0x3A */ I32Store8(u32, u32),
    /* 0x3B */ I32Store16(u32, u32),
    /* 0x3C */ I64Store8(u32, u32),
    /* 0x3D */ I64Store16(u32, u32),
    /* 0x3E */ I64Store32(u32, u32),
    // Memory management
    /* 0x3F */ MemSize(idx::Mem),
    /* 0x40 */ MemGrow(idx::Mem),
    // Numeric
    /* 0x41 */ ConstI32(i32),
    /* 0x42 */ ConstI64(i64),
    /* 0x43 */ ConstF32(f32),
    /* 0x44 */ ConstF64(f64),
    // Numeric without immediate -- i32
    /* 0x45 */ I32Eqz,
    /* 0x46 */ I32Eq,
    /* 0x47 */ I32Ne,
    /* 0x48 */ I32LtS,
    /* 0x49 */ I32LtU,
    /* 0x4A */ I32GtS,
    /* 0x4B */ I32GtU,
    /* 0x4C */ I32LeS,
    /* 0x4D */ I32LeU,
    /* 0x4E */ I32GeS,
    /* 0x4F */ I32GeU,
    // Numeric without immediate -- i64
    /* 0x50 */ I64Eqz,
    /* 0x51 */ I64Eq,
    /* 0x52 */ I64Ne,
    /* 0x53 */ I64LtS,
    /* 0x54 */ I64LtU,
    /* 0x55 */ I64GtS,
    /* 0x56 */ I64GtU,
    /* 0x57 */ I64LeS,
    /* 0x58 */ I64LeU,
    /* 0x59 */ I64GeS,
    /* 0x5A */ I64GeU,
    // Numeric without immediate -- f32
    /* 0x5B */ F32Eq,
    /* 0x5C */ F32Ne,
    /* 0x5D */ F32Lt,
    /* 0x5E */ F32Gt,
    /* 0x5F */ F32Le,
    /* 0x60 */ F32Ge,
    // Numeric without immediate -- f64
    /* 0x61 */ F64Eq,
    /* 0x62 */ F64Ne,
    /* 0x63 */ F64Lt,
    /* 0x64 */ F64Gt,
    /* 0x65 */ F64Le,
    /* 0x66 */ F64Ge,
    // Numeric without immediate -- i32 cont.
    /* 0x67 */ I32Clz,
    /* 0x68 */ I32Ctz,
    /* 0x69 */ I32Popcnt,
    /* 0x6A */ I32Add,
    /* 0x6B */ I32Sub,
    /* 0x6C */ I32Mul,
    /* 0x6D */ I32DivS,
    /* 0x6E */ I32DivU,
    /* 0x6F */ I32RemS,
    /* 0x70 */ I32RemU,
    /* 0x71 */ I32And,
    /* 0x72 */ I32Or,
    /* 0x73 */ I32Xor,
    /* 0x74 */ I32Shl,
    /* 0x75 */ I32ShrS,
    /* 0x76 */ I32ShrU,
    /* 0x77 */ I32Rotl,
    /* 0x78 */ I32Rotr,
    // Numeric without immediate -- i64 cont.
    /* 0x79 */ I64Clz,
    /* 0x7A */ I64Ctz,
    /* 0x7B */ I64Popcnt,
    /* 0x7C */ I64Add,
    /* 0x7D */ I64Sub,
    /* 0x7E */ I64Mul,
    /* 0x7F */ I64DivS,
    /* 0x80 */ I64DivU,
    /* 0x81 */ I64RemS,
    /* 0x82 */ I64RemU,
    /* 0x83 */ I64And,
    /* 0x84 */ I64Or,
    /* 0x85 */ I64Xor,
    /* 0x86 */ I64Shl,
    /* 0x87 */ I64ShrS,
    /* 0x88 */ I64ShrU,
    /* 0x89 */ I64Rotl,
    /* 0x8A */ I64Rotr,
    // Numeric without immediate -- f32 cont.
    /* 0x8B */ F32Abs,
    /* 0x8C */ F32Neg,
    /* 0x8D */ F32Ceil,
    /* 0x8E */ F32Floor,
    /* 0x8F */ F32Trunc,
    /* 0x90 */ F32Nearest,
    /* 0x91 */ F32Sqrt,
    /* 0x92 */ F32Add,
    /* 0x93 */ F32Sub,
    /* 0x94 */ F32Mul,
    /* 0x95 */ F32Div,
    /* 0x96 */ F32Min,
    /* 0x97 */ F32Max,
    /* 0x98 */ F32Copysign,
    // Numeric without immediate -- f64 cont.
    /* 0x99 */ F64Abs,
    /* 0x9A */ F64Neg,
    /* 0x9B */ F64Ceil,
    /* 0x9C */ F64Floor,
    /* 0x9D */ F64Trunc,
    /* 0x9E */ F64Nearest,
    /* 0x9F */ F64Sqrt,
    /* 0xA0 */ F64Add,
    /* 0xA1 */ F64Sub,
    /* 0xA2 */ F64Mul,
    /* 0xA3 */ F64Div,
    /* 0xA4 */ F64Min,
    /* 0xA5 */ F64Max,
    /* 0xA6 */ F64Copysign,
    // Numeric without immediate -- conversion
    /* 0xA7 */ I32WrapI64,
    /* 0xA8 */ I32TruncF32S,
    /* 0xA9 */ I32TruncF32U,
    /* 0xAA */ I32TruncF64S,
    /* 0xAB */ I32TruncF64U,
    /* 0xAC */ I64ExtendI32S,
    /* 0xAD */ I64ExtendI32U,
    /* 0xAE */ I64TruncF32S,
    /* 0xAF */ I64TruncF32U,
    /* 0xB0 */ I64TruncF64S,
    /* 0xB1 */ I64TruncF64U,
    /* 0xB2 */ F32ConvertI32S,
    /* 0xB3 */ F32ConvertI32U,
    /* 0xB4 */ F32ConvertI64S,
    /* 0xB5 */ F32ConvertI64U,
    /* 0xB6 */ F32DemoteF64,
    /* 0xB7 */ F64ConvertI32S,
    /* 0xB8 */ F64ConvertI32U,
    /* 0xB9 */ F64ConvertI64S,
    /* 0xBA */ F64ConvertI64U,
    /* 0xBB */ F64PromoteF32,
    /* 0xBC */ I32ReinterpretF32,
    /* 0xBD */ I64ReinterpretF64,
    /* 0xBE */ F32ReinterpretI32,
    /* 0xBF */ F64ReinterpretI64,
        // ... reserved ...
}