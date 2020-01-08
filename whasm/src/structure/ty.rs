#[derive(Debug, PartialEq)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
pub enum Val {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, PartialEq)]
pub enum Elem {
    FuncRef,
}

#[derive(Debug, PartialEq)]
pub struct RetVal (pub Vec<Val>);

#[derive(Debug, PartialEq)]
pub struct Func {
    pub params: Vec<Val>,
    pub results: Vec<Val>,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub lim: Limits,
    pub elem: Elem,
}

#[derive(Debug, PartialEq)]
pub struct Mem {
    pub lim: Limits,
}

#[derive(Debug, PartialEq)]
pub enum Mut { Const, Var }

#[derive(Debug, PartialEq)]
pub struct Global {
    pub mt: Mut,
    pub val: Val,
}