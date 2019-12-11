//! This module provides the building blocks to declaratively define the grammar in the
//! [WebAssembly Specification](https://webassembly.github.io/spec/).
//! 
//! With the current functionalities in `whasm::grammar::core`, all the grammar elements in release
//! 1.0 of the specification can be expressed declaratively.

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Trait that describes an element of the WebAssembly grammar (as defined in
/// the [WebAssembly Specification](https://webassembly.github.io/spec/)).
/// Objects that implement this trait can be deserialized from an `Iterator<u8>`.
/// 
/// This trait requires implementing the `deserialize` method.
/// However, in most cases the `Grammar` derive macro can be used instead.
/// The derive macro defines new grammars declaratively, making it the preferred
/// approach to define new grammars.
/// 
/// # Example
/// 
/// ```
/// # use whasm::grammar::*;
/// struct AsciiChar(char);
/// impl Grammar for AsciiChar {
///     fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
///         let Byte(byte) = deserialize(iter)?;
///         if byte & 0x80 != 0 {
///             Err("The most significant bit of an ASCII character must be 0.")?;
///         }
///         Ok(AsciiChar(byte as char))
///     }
/// }
/// ```
///
pub trait Grammar
where Self: Sized {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self>;
}

/// Convenience function to deserialize an object that implements the `Grammar` trait.
pub fn deserialize<T: Grammar, Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<T> {
    <T as Grammar>::deserialize(iter)
}

pub mod unsigned;
pub mod signed;
pub mod byte;
pub mod vec;
pub mod string;
pub mod float;
pub mod array;
pub mod error;
pub mod list;
pub mod derive;

pub use self::byte::Byte;
pub use self::list::List;
pub use self::error::Error;
pub use self::derive::Grammar;