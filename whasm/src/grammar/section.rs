use super::*;
use super::ty::*;
use super::idx::*;
use super::instruction::*;

#[derive(Debug, PartialEq, Grammar)]
#[sized]
pub enum Section {
    #[discriminant(0x00)] Custom (CustomSection),
    #[discriminant(0x01)] Type (TypeSection),
    #[discriminant(0x02)] Import (ImportSection),
    #[discriminant(0x03)] Function (FuncSection),
    #[discriminant(0x04)] Table (TableSection),
    #[discriminant(0x05)] Memory (MemorySection),
    #[discriminant(0x06)] Global (GlobalSection),
    #[discriminant(0x07)] Export (ExportSection),
    #[discriminant(0x08)] Start (StartSection),
    #[discriminant(0x09)] Element (ElementSection),
    #[discriminant(0x0A)] Code (CodeSection),
    #[discriminant(0x0B)] Data (DataSection),
}

#[derive(Grammar, PartialEq)]
pub struct CustomSection {
    pub name: String,
    pub payload: List<Byte>,
}

impl std::fmt::Debug for CustomSection {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CustomSection")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Debug, PartialEq, Grammar)]
pub struct TypeSection {
    pub types: Vec<FuncType>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct ImportSection {
    pub imports: Vec<Import>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, PartialEq, Grammar)]
pub enum ImportDesc {
    #[discriminant(0x00)] Func(TypeIdx),
    #[discriminant(0x01)] Table(TableType),
    #[discriminant(0x02)] Mem(MemType),
    #[discriminant(0x03)] Global(GlobalType),
}

#[derive(Debug, PartialEq, Grammar)]
pub struct FuncSection {
    pub funcs: Vec<TypeIdx>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct TableSection {
    pub tables: Vec<Table>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Table {
    pub ty: TableType,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct MemorySection {
    pub mems: Vec<Memory>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Memory {
    pub ty: MemType,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct GlobalSection {
    pub globals: Vec<Global>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Global {
    pub ty: GlobalType,
    pub init: Expression,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct ExportSection {
    pub exports: Vec<Export>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Debug, PartialEq, Grammar)]
pub enum ExportDesc {
    #[discriminant(0x00)] Func(TypeIdx),
    #[discriminant(0x01)] Table(TableIdx),
    #[discriminant(0x02)] Mem(MemIdx),
    #[discriminant(0x03)] Global(GlobalIdx),
}

#[derive(Debug, PartialEq, Grammar)]
pub struct StartSection {
    pub start: FuncIdx,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct ElementSection {
    pub elem: Vec<Element>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Element {
    pub table: TableIdx,
    pub offset: Expression,
    pub init: Vec<FuncIdx>,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct CodeSection {
    pub funcs: Vec<Function>,
}

#[derive(Debug, PartialEq, Grammar)]
#[sized]
pub struct Function {
    pub locals: Vec<Locals>,
    pub code: Expression,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct Locals {
    pub n: u32,
    pub ty: ValType,
}

#[derive(Debug, PartialEq, Grammar)]
pub struct DataSection {
    pub data: Vec<Data>,
}

#[derive(PartialEq, Grammar)]
pub struct Data {
    pub data: MemIdx,
    pub offset: Expression,
    pub init: Vec<Byte>,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Data")
            .field("data", &self.data)
            .field("offset", &self.offset)
            .finish()
    }
}