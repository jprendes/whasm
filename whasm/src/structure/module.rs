use super::{ty, idx, instr};

#[derive(Debug, PartialEq)]
pub struct Module {
    pub types: Vec<ty::Func>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elem: Vec<Elem>,
    pub data: Vec<Data>,
    pub start: Option<Start>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub ty: idx::Type,
    pub locals: Vec<ty::Val>,
    pub body: instr::Expr,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub ty: ty::Table,
}

#[derive(Debug, PartialEq)]
pub struct Mem {
    pub ty: ty::Mem,
}

#[derive(Debug, PartialEq)]
pub struct Global {
    pub ty: ty::Global,
    pub init: instr::Expr,
}

#[derive(Debug, PartialEq)]
pub struct Elem {
    pub table: idx::Table,
    pub offset: instr::Expr,
    pub init: Vec<idx::Func>,
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub mem: idx::Mem,
    pub offset: instr::Expr,
    pub init: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct Start {
    pub func: idx::Func,
}

pub mod desc {
    use super::{ty, idx};

    #[derive(Debug, PartialEq)]
    pub enum Import {
        Func(idx::Type),
        Table(ty::Table),
        Mem(ty::Mem),
        Global(ty::Global),
    }

    #[derive(Debug, PartialEq)]
    pub enum Export {
        Func(idx::Func),
        Table(idx::Table),
        Mem(idx::Mem),
        Global(idx::Global),
    }
}

#[derive(Debug, PartialEq)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: desc::Import,
}

#[derive(Debug, PartialEq)]
pub struct Export {
    pub name: String,
    pub desc: desc::Export,
}