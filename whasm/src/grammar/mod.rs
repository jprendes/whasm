//! This module defines all the grammar elements defined in the release 1.0 of the
//! [WebAssembly Specification](https://webassembly.github.io/spec/). It also defines the
//! deserialization of these elements from the WebAssembly binary representation.
//! 
//! # Example
//! 
//! Deserializing the smallest possible WebAssembly file, a `Module` with no `Section`s.
//! 
//! ```
//! # use whasm::grammar::prelude::*;
//! let mut iter = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00].iter().copied();
//! let Module{ sections, .. } = deserialize(&mut iter).unwrap();
//! assert_eq!(sections, []);
//! ```
//! 
//! A slightly more complex WebAssembly file, which exports a function named `main` that returns an `i32` with the value of `42`.
//!  
//! ```
//! # use whasm::grammar::prelude::*;
//! let mut iter = [
//!     0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x01,
//!     0x60, 0x00, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01,
//!     0x04, 0x6D, 0x61, 0x69, 0x6E, 0x00, 0x00, 0x0a, 0x0a, 0x01, 0x84,
//!     0x80, 0x80, 0x80, 0x00, 0x00, 0x41, 0x2a, 0x0b
//! ].iter().copied();
//! let Module{ sections, .. } = deserialize(&mut iter).unwrap();
//! assert_eq!(sections, [
//!     Section::Type(TypeSection { types: vec![
//!         FuncType::Func {
//!             params: vec![],
//!             results: vec![ ValType::I32 ]
//!         }
//!     ] }),
//!     Section::Function(FuncSection { funcs: vec![
//!         TypeIdx(0)
//!     ] }),
//!     Section::Export(ExportSection { exports: vec![
//!         Export {
//!             name: String::from("main"),
//!             desc: ExportDesc::Func(TypeIdx(0))
//!         }
//!     ] }),
//!     Section::Code(CodeSection { funcs: vec![
//!         Function {
//!             locals: VecVec(vec![]),
//!             code: Expression(vec![
//!                 Instruction::ConstI32(42)
//!             ])
//!         }
//!     ] })
//! ]);
//! ```

pub mod core;
pub use self::core::*;

pub mod idx;
pub mod instr;
pub mod module;
pub mod section;
pub mod ty;

pub mod prelude;

#[cfg(test)]
mod test {
    use crate::grammar::prelude::*;

    #[test]
    fn can_deserialize_was_file() {
        let mut iter = [
            0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x85, 0x80, 0x80, 0x80, 0x00, 0x01, 0x60,
            0x00, 0x01, 0x7f, 0x03, 0x82, 0x80, 0x80, 0x80, 0x00, 0x01, 0x00, 0x04, 0x84, 0x80, 0x80, 0x80,
            0x00, 0x01, 0x70, 0x00, 0x00, 0x06, 0x81, 0x80, 0x80, 0x80, 0x00, 0x00, 0x07, 0x88, 0x80, 0x80,
            0x80, 0x00, 0x01, 0x04, 0x74, 0x65, 0x73, 0x74, 0x00, 0x00, 0x0a, 0x8a, 0x80, 0x80, 0x80, 0x00,
            0x01, 0x84, 0x80, 0x80, 0x80, 0x00, 0x00, 0x41, 0x2a, 0x0b,
        ].iter().copied();
        let result: Module = deserialize(&mut iter).unwrap();
        assert_eq!(result.magic, ['\0', 'a', 's', 'm']);
        assert_eq!(result.version, [1, 0, 0, 0]);
        assert_eq!(result.sections, [
            Section::Type(TypeSection { types: vec![
                FuncType::Func {
                    params: vec![],
                    results: vec![ ValType::I32 ]
                }
            ] }),
            Section::Function(FuncSection { funcs: vec![
                TypeIdx(0)
            ] }),
            Section::Table(TableSection { tables: vec![
                Table { ty: TableType {
                    et: ElemType::FuncRef,
                    lim: Limits::Min(0)
                } }
            ] }),
            Section::Global(GlobalSection { globals: vec![
            ] }),
            Section::Export(ExportSection { exports: vec![
                Export {
                    name: String::from("test"),
                    desc: ExportDesc::Func(TypeIdx(0))
                }
            ] }),
            Section::Code(CodeSection { funcs: vec![
                Function {
                    locals: VecVec(vec![]),
                    code: Expression(vec![
                        Instruction::ConstI32(42)
                    ])
                }
            ] })
        ]);
    }
}
