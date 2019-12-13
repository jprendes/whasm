use crate as whasm;
use self::whasm::grammar::{ *, idx::*, ty::* };

#[derive(Debug, PartialEq, Grammar)]
pub enum Instruction {
    // Control flow
    #[discriminant(0x00)] Unreachable,
    #[discriminant(0x01)] Nop,
    #[discriminant(0x02)] Block(BlockInstr),
    #[discriminant(0x03)] Loop(BlockInstr),
    #[discriminant(0x04)] If(IfInstr),
    #[discriminant(0x05)] Else,
        // ... reserved ...
    #[discriminant(0x0B)] End,
    #[discriminant(0x0C)] Br(LabelIdx),
    #[discriminant(0x0D)] BrIf(LabelIdx),
    #[discriminant(0x0E)] BrTable(Vec<LabelIdx>, LabelIdx),
    #[discriminant(0x0F)] Return,
    #[discriminant(0x10)] Call(FuncIdx),
    #[discriminant(0x11)] CallIndirect(FuncIdx, TableIdx),
        // ... reserved ...
    // Parametric
    #[discriminant(0x1A)] Drop,
    #[discriminant(0x1B)] Select,
        // ... reserved ...
    // Variable
    #[discriminant(0x20)] LocalGet(LocalIdx),
    #[discriminant(0x21)] LocalSet(LocalIdx),
    #[discriminant(0x22)] LocalTee(LocalIdx),
    #[discriminant(0x23)] GlobalGet(GlobalIdx),
    #[discriminant(0x24)] GlobalSet(GlobalIdx),
        // ... reserved ...
    // Memory load
    #[discriminant(0x28)] I32Load(MemArg),
    #[discriminant(0x29)] I64Load(MemArg),
    #[discriminant(0x2A)] F32Load(MemArg),
    #[discriminant(0x2B)] F64Load(MemArg),
    #[discriminant(0x2C)] I32Load8S(MemArg),
    #[discriminant(0x2D)] I32Load8U(MemArg),
    #[discriminant(0x2E)] I32Load16S(MemArg),
    #[discriminant(0x2F)] I32Load16U(MemArg),
    #[discriminant(0x30)] I64Load8S(MemArg),
    #[discriminant(0x31)] I64Load8U(MemArg),
    #[discriminant(0x32)] I64Load16S(MemArg),
    #[discriminant(0x33)] I64Load16U(MemArg),
    #[discriminant(0x34)] I64Load32S(MemArg),
    #[discriminant(0x35)] I64Load32U(MemArg),
    // Memory store
    #[discriminant(0x36)] I32Store(MemArg),
    #[discriminant(0x37)] I64Store(MemArg),
    #[discriminant(0x38)] F32Store(MemArg),
    #[discriminant(0x39)] F64Store(MemArg),
    #[discriminant(0x3A)] I32Store8(MemArg),
    #[discriminant(0x3B)] I32Store16(MemArg),
    #[discriminant(0x3C)] I64Store8(MemArg),
    #[discriminant(0x3D)] I64Store16(MemArg),
    #[discriminant(0x3E)] I64Store32(MemArg),
    // Memory management
    #[discriminant(0x3F)] MemSize(MemIdx),
    #[discriminant(0x40)] MemGrow(MemIdx),
    // Numeric
    #[discriminant(0x41)] ConstI32(i32),
    #[discriminant(0x42)] ConstI64(i64),
    #[discriminant(0x43)] ConstF32(f32),
    #[discriminant(0x44)] ConstF64(f64),
    // Numeric without immediate -- i32
    #[discriminant(0x45)] I32Eqz,
    #[discriminant(0x46)] I32Eq,
    #[discriminant(0x47)] I32Ne,
    #[discriminant(0x48)] I32LtS,
    #[discriminant(0x49)] I32LtU,
    #[discriminant(0x4A)] I32GtS,
    #[discriminant(0x4B)] I32GtU,
    #[discriminant(0x4C)] I32LeS,
    #[discriminant(0x4D)] I32LeU,
    #[discriminant(0x4E)] I32GeS,
    #[discriminant(0x4F)] I32GeU,
    // Numeric without immediate -- i64
    #[discriminant(0x50)] I64Eqz,
    #[discriminant(0x51)] I64Eq,
    #[discriminant(0x52)] I64Ne,
    #[discriminant(0x53)] I64LtS,
    #[discriminant(0x54)] I64LtU,
    #[discriminant(0x55)] I64GtS,
    #[discriminant(0x56)] I64GtU,
    #[discriminant(0x57)] I64LeS,
    #[discriminant(0x58)] I64LeU,
    #[discriminant(0x59)] I64GeS,
    #[discriminant(0x5A)] I64GeU,
    // Numeric without immediate -- f32
    #[discriminant(0x5B)] F32Eq,
    #[discriminant(0x5C)] F32Ne,
    #[discriminant(0x5D)] F32Lt,
    #[discriminant(0x5E)] F32Gt,
    #[discriminant(0x5F)] F32Le,
    #[discriminant(0x60)] F32Ge,
    // Numeric without immediate -- f64
    #[discriminant(0x61)] F64Eq,
    #[discriminant(0x62)] F64Ne,
    #[discriminant(0x63)] F64Lt,
    #[discriminant(0x64)] F64Gt,
    #[discriminant(0x65)] F64Le,
    #[discriminant(0x66)] F64Ge,
    // Numeric without immediate -- i32 cont.
    #[discriminant(0x67)] I32Clz,
    #[discriminant(0x68)] I32Ctz,
    #[discriminant(0x69)] I32Popcnt,
    #[discriminant(0x6A)] I32Add,
    #[discriminant(0x6B)] I32Sub,
    #[discriminant(0x6C)] I32Mul,
    #[discriminant(0x6D)] I32DivS,
    #[discriminant(0x6E)] I32DivU,
    #[discriminant(0x6F)] I32RemS,
    #[discriminant(0x70)] I32RemU,
    #[discriminant(0x71)] I32And,
    #[discriminant(0x72)] I32Or,
    #[discriminant(0x73)] I32Xor,
    #[discriminant(0x74)] I32Shl,
    #[discriminant(0x75)] I32ShrS,
    #[discriminant(0x76)] I32ShrU,
    #[discriminant(0x77)] I32Rotl,
    #[discriminant(0x78)] I32Rotr,
    // Numeric without immediate -- i64 cont.
    #[discriminant(0x79)] I64Clz,
    #[discriminant(0x7A)] I64Ctz,
    #[discriminant(0x7B)] I64Popcnt,
    #[discriminant(0x7C)] I64Add,
    #[discriminant(0x7D)] I64Sub,
    #[discriminant(0x7E)] I64Mul,
    #[discriminant(0x7F)] I64DivS,
    #[discriminant(0x80)] I64DivU,
    #[discriminant(0x81)] I64RemS,
    #[discriminant(0x82)] I64RemU,
    #[discriminant(0x83)] I64And,
    #[discriminant(0x84)] I64Or,
    #[discriminant(0x85)] I64Xor,
    #[discriminant(0x86)] I64Shl,
    #[discriminant(0x87)] I64ShrS,
    #[discriminant(0x88)] I64ShrU,
    #[discriminant(0x89)] I64Rotl,
    #[discriminant(0x8A)] I64Rotr,
    // Numeric without immediate -- f32 cont.
    #[discriminant(0x8B)] F32Abs,
    #[discriminant(0x8C)] F32Neg,
    #[discriminant(0x8D)] F32Ceil,
    #[discriminant(0x8E)] F32Floor,
    #[discriminant(0x8F)] F32Trunc,
    #[discriminant(0x90)] F32Nearest,
    #[discriminant(0x91)] F32Sqrt,
    #[discriminant(0x92)] F32Add,
    #[discriminant(0x93)] F32Sub,
    #[discriminant(0x94)] F32Mul,
    #[discriminant(0x95)] F32Div,
    #[discriminant(0x96)] F32Min,
    #[discriminant(0x97)] F32Max,
    #[discriminant(0x98)] F32Copysign,
    // Numeric without immediate -- f64 cont.
    #[discriminant(0x99)] F64Abs,
    #[discriminant(0x9A)] F64Neg,
    #[discriminant(0x9B)] F64Ceil,
    #[discriminant(0x9C)] F64Floor,
    #[discriminant(0x9D)] F64Trunc,
    #[discriminant(0x9E)] F64Nearest,
    #[discriminant(0x9F)] F64Sqrt,
    #[discriminant(0xA0)] F64Add,
    #[discriminant(0xA1)] F64Sub,
    #[discriminant(0xA2)] F64Mul,
    #[discriminant(0xA3)] F64Div,
    #[discriminant(0xA4)] F64Min,
    #[discriminant(0xA5)] F64Max,
    #[discriminant(0xA6)] F64Copysign,
    // Numeric without immediate -- conversion
    #[discriminant(0xA7)] I32WrapI64,
    #[discriminant(0xA8)] I32TruncF32S,
    #[discriminant(0xA9)] I32TruncF32U,
    #[discriminant(0xAA)] I32TruncF64S,
    #[discriminant(0xAB)] I32TruncF64U,
    #[discriminant(0xAC)] I64ExtendI32S,
    #[discriminant(0xAD)] I64ExtendI32U,
    #[discriminant(0xAE)] I64TruncF32S,
    #[discriminant(0xAF)] I64TruncF32U,
    #[discriminant(0xB0)] I64TruncF64S,
    #[discriminant(0xB1)] I64TruncF64U,
    #[discriminant(0xB2)] F32ConvertI32S,
    #[discriminant(0xB3)] F32ConvertI32U,
    #[discriminant(0xB4)] F32ConvertI64S,
    #[discriminant(0xB5)] F32ConvertI64U,
    #[discriminant(0xB6)] F32DemoteF64,
    #[discriminant(0xB7)] F64ConvertI32S,
    #[discriminant(0xB8)] F64ConvertI32U,
    #[discriminant(0xB9)] F64ConvertI64S,
    #[discriminant(0xBA)] F64ConvertI64U,
    #[discriminant(0xBB)] F64PromoteF32,
    #[discriminant(0xBC)] I32ReinterpretF32,
    #[discriminant(0xBD)] I64ReinterpretF64,
    #[discriminant(0xBE)] F32ReinterpretI32,
    #[discriminant(0xBF)] F64ReinterpretI64,
        // ... reserved ...
}

#[derive(Debug, PartialEq, Grammar)]
pub enum LoadInstr {
    #[discriminant(0x28)] I32(MemArg),
    #[discriminant(0x29)] I64(MemArg),
    #[discriminant(0x2A)] F32(MemArg),
    #[discriminant(0x2B)] F64(MemArg),
}

#[derive(Debug, PartialEq, Grammar)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct BlockInstr {
    pub ty: BlockType,
    pub instr: Expression,
}

#[derive(Debug, PartialEq)]
pub struct IfInstr {
    pub ty: BlockType,
    pub then_instr: Expression,
    pub else_instr: Expression,
}

impl Grammar for IfInstr {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        let ty = deserialize(iter)?;
        let (then_instr, else_instr) = match deserialize(iter)? {
            InstrList(then_instr, Instruction::End) => (then_instr, vec![]),
            InstrList(then_instr, Instruction::Else) => {
                match deserialize(iter)? {
                    InstrList(else_instr, Instruction::End) => (then_instr, else_instr),
                    _ => Err(Error::UnexpectedInstruction { ident: "Else".into() })?,
                }
            },
            _ => unreachable!()
        };
        Ok(IfInstr {
            ty,
            then_instr: Expression(then_instr),
            else_instr: Expression(else_instr),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression(pub Vec<Instruction>);

impl Grammar for Expression {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok(Expression(match deserialize(iter)? {
            InstrList(instr, Instruction::End) => instr,
            _ => Err(Error::UnexpectedInstruction { ident: "Else".into() })?,
        }))
    }
}

#[derive(Debug, PartialEq)]
struct InstrList(pub Vec<Instruction>, Instruction);

impl Grammar for InstrList {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        let mut instr = vec![];
        let last;
        loop { match deserialize(iter)? {
            Instruction::End => { last = Instruction::End; break; },
            Instruction::Else => { last = Instruction::Else; break; },
            ins => instr.push(ins),
        } };
        Ok(InstrList(instr, last))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_deserialize_expression() {
        let mut iter = [0x01, 0x01, 0x01, 0x0B].iter().copied();
        let result: Expression = deserialize(&mut iter).unwrap();
        assert_eq!(result, Expression(vec![Instruction::Nop, Instruction::Nop, Instruction::Nop]));
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_expression_with_else_instruction() {
        let mut iter = [0x01, 0x01, 0x01, 0x05, 0x01, 0x01, 0x0B].iter().copied();
        let _: Expression = deserialize(&mut iter).unwrap();
    }

    #[test]
    fn can_deserialize_if_instruction() {
        let mut iter = [0x40, 0x01, 0x01, 0x05, 0x01, 0x0B].iter().copied();
        let result: IfInstr = deserialize(&mut iter).unwrap();
        assert_eq!(result, IfInstr {
            ty: BlockType::Empty,
            then_instr: Expression(vec![Instruction::Nop, Instruction::Nop]),
            else_instr: Expression(vec![Instruction::Nop]),
        });
    }

    #[test]
    fn can_deserialize_if_instruction_without_else_branch() {
        let mut iter = [0x40, 0x01, 0x01, 0x01, 0x0B].iter().copied();
        let result: IfInstr = deserialize(&mut iter).unwrap();
        assert_eq!(result, IfInstr {
            ty: BlockType::Empty,
            then_instr: Expression(vec![Instruction::Nop, Instruction::Nop, Instruction::Nop]),
            else_instr: Expression(vec![]),
        });
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_if_instruction_without_two_else_instructions() {
        let mut iter = [0x40, 0x01, 0x01, 0x05, 0x01, 0x05].iter().copied();
        let _: IfInstr = deserialize(&mut iter).unwrap();
    }
}