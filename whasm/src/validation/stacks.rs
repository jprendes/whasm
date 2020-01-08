use crate::validation::{Result, Error};
use crate::structure::ty;

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum Operand {
    Unknown,
    Val(ty::Val),
}

impl From<ty::Val> for Operand {
    fn from(val: ty::Val) -> Operand {
        Operand::Val(val)
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Frame<'a> {
    pub label: &'a [ty::Val],
    pub out: &'a [ty::Val],
    pub height: usize,
    pub unreachable: bool,
}

#[derive(Default)]
pub struct Stacks<'a> {
    pub operands: Vec<Operand>,
    pub frames: Vec<Frame<'a>>,
}

impl<'a> Stacks<'a> {
    pub fn push_operand(&mut self, op: Operand) {
        self.operands.push(op);
    }

    pub fn pop_operand(&mut self, op: Operand) -> Result<Operand> {
        let frame = self.frames.last().ok_or(Error::UnexpectedEndOfFile)?;
        if self.operands.len() == frame.height && frame.unreachable {
            return Ok(op);
        }
        if self.operands.len() == frame.height {
            return Err(Error::UnexpectedEndOfFile)
        }
        let val = self.operands.pop().unwrap();
        match (val, op) {
            (Operand::Unknown, _) => Ok(op),
            (_, Operand::Unknown) => Ok(val),
            (_, _) if val == op => Ok(val),
            (_, _) => Err(Error::UnexpectedEndOfFile),
        }
    }

    pub fn push_operands(&mut self, ops: &[ty::Val]) {
        for op in ops.iter() {
            self.push_operand(Operand::Val(*op));
        }
    }

    pub fn pop_operands(&mut self, ops: &[ty::Val]) -> Result<()> {
        for op in ops.iter().rev() {
            self.pop_operand(Operand::Val(*op))?;
        }
        Ok(())
    }

    pub fn push_frame(&mut self, label: &'a [ty::Val], out: &'a [ty::Val]) {
        self.frames.push(Frame {
            label,
            out,
            height: self.operands.len(),
            unreachable: false,
        })
    }

    pub fn pop_frame(&mut self) -> Result<&'a [ty::Val]> {
        let frame = self.frames.last().ok_or(Error::UnexpectedEndOfFile)?;
        let out = frame.out;
        let height = frame.height;
        self.pop_operands(&out[..])?;
        if self.operands.len() != height {
            return Err(Error::UnexpectedEndOfFile);
        }
        self.frames.pop().ok_or(Error::UnexpectedEndOfFile)?;
        Ok(out)
    }

    pub fn unreachable(&mut self) -> Result<()> {
        let frame = self.frames.last().ok_or(Error::UnexpectedEndOfFile)?;
        let height = frame.height;
        self.operands.truncate(height);
        self.frames.last_mut().unwrap().unreachable = true;
        Ok(())
    }
}