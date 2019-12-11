use crate as whasm;
use self::whasm::grammar::*;

#[derive(Clone, PartialEq, Copy, Debug, Grammar)]
pub enum ValType {
    #[discriminant(0x7F)] I32,
    #[discriminant(0x7E)] I64,
    #[discriminant(0x7D)] F32,
    #[discriminant(0x7C)] F64,
}

#[derive(Debug, PartialEq, Grammar)]
pub enum BlockType {
    #[discriminant(0x40)] Empty,
    #[forward] Value(ValType),
}

#[derive(Debug, PartialEq, Grammar)]
pub enum FuncType {
    #[discriminant(0x60)] Func {
        params: Vec<ValType>,
        results: Vec<ValType>,
    },
}

#[derive(Debug, PartialEq, Grammar)]
pub enum Limits {
    #[discriminant(0x00)] Min(u32),
    #[discriminant(0x01)] MinMax(u32,u32),
}

#[derive(Debug, PartialEq, Grammar)]
pub struct MemType {
    pub lim: Limits,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct TableType {
    pub et: ElemType,
    pub lim: Limits,
}

#[derive(Debug, PartialEq, Grammar)]
pub enum ElemType {
    #[discriminant(0x70)]
    FuncRef
}

#[derive(Debug, PartialEq, Grammar)]
pub struct GlobalType {
    pub t: ValType,
    pub m: Mut,
}

#[derive(Debug, PartialEq, Grammar)]
pub enum Mut {
    #[discriminant(0x00)] Const,
    #[discriminant(0x01)] Var,
}