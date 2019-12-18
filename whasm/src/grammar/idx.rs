use crate as whasm;
use self::whasm::grammar::*;

#[derive(Debug, PartialEq, Grammar)]
pub struct TypeIdx(pub u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct FuncIdx(pub u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct TableIdx(pub u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct MemIdx(pub u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct GlobalIdx(pub u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct LocalIdx(pub u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct LabelIdx(pub u32);