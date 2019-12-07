use super::*;
use super::idx::*;
use super::ty::*;

#[derive(Debug, PartialEq, Grammar)]
pub enum Instruction {
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
    #[discriminant(0x1A)] Drop,
    #[discriminant(0x1B)] Select,
    // ... reserved ...
    #[discriminant(0x20)] LocalGet(LocalIdx),
    #[discriminant(0x21)] LocalSet(LocalIdx),
    #[discriminant(0x22)] LocalTee(LocalIdx),
    #[discriminant(0x23)] GlobalGet(GlobalIdx),
    #[discriminant(0x24)] GlobalSet(GlobalIdx),
    // ... reserved ...
    #[discriminant(0x28..=0x35)] Load(MemArg), // Overloaded, should distinguish cases
    #[discriminant(0x36..=0x3E)] Store(MemArg), // Overloaded, should distinguish cases
    #[discriminant(0x3F)] MemSize(MemIdx),
    #[discriminant(0x40)] MemGrow(MemIdx),
    #[discriminant(0x41)] ConstI32(i32),
    #[discriminant(0x42)] ConstI64(i64),
    #[discriminant(0x43)] ConstF32(f32),
    #[discriminant(0x44)] ConstF64(f64),
    #[discriminant(0x45..=0xBF)] Numeric, // Overloaded, should distinguish cases
    // ... reserved ...
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