use super::*;

#[derive(Debug, PartialEq, Grammar)]
pub struct TypeIdx(u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct FuncIdx(u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct TableIdx(u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct MemIdx(u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct GlobalIdx(u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct LocalIdx(u32);

#[derive(Debug, PartialEq, Grammar)]
pub struct LabelIdx(u32);