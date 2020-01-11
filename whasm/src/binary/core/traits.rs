//! This module defines a trait used during parsing of WebAssembly binary files.

use crate::binary::Result;

/// This is a trait for types that can be used as input for parsing WebAssembly binary files.
/// The trait is automatically applies to any type implementing `Iterator<Item=u8>`.
/// 
/// This trait defines a `parse` method, which parses an element from the iterator.
/// The method is defined to any type that implements the `WasmBinaryParse` trait.
/// 
/// # Example
/// 
/// The `whasm::binary::Byte` implements the `WasmBinaryParse`. It can be parsed from an iterator
/// using the `parse` method.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, Byte};
/// let mut iter = [0x2A].iter().copied();
/// let result = iter.parse::<Byte>().unwrap();
/// assert_eq!(result, 42)
/// ```
pub trait WasmBinary: Iterator<Item=u8>
where Self: std::marker::Sized {
    fn parse<T: WasmBinaryParse>(&mut self) -> Result<T> {
        <T as WasmBinaryParse>::parse(self)
    }
}

impl<T> WasmBinary for T
where T: Iterator<Item=u8> {
}

/// This is a trait for parsing elements in a WebAssembly binary file.
/// 
/// This trait defines the `parse` method, which takes a `WasmBinary` and returns a `Result<Self>`.
/// Types that implement this trait can be parsed from a `WasmBinary` using its `parse` method.
/// 
/// # Example
/// 
/// Parsing an enum variant based on a binary id obtained from the input file.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, WasmBinaryParse, Result, Byte};
/// #[derive(Debug, PartialEq)]
/// enum MyEnum {
///     Variant1,
///     Variant2,
///     Variant3,
///     Unknown,
/// };
/// impl WasmBinaryParse for MyEnum {
///     fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
///         let Byte(id) = bin.parse()?;
///         match id {
///             0x01 => Ok(MyEnum::Variant1),
///             0x02 => Ok(MyEnum::Variant2),
///             0x03 => Ok(MyEnum::Variant3),
///             _ => Ok(MyEnum::Unknown),
///         }
///     }
/// }
/// 
/// let mut iter = [0x03].iter().copied();
/// let result: MyEnum = iter.parse().unwrap();
/// assert_eq!(result, MyEnum::Variant3)
/// ```
pub trait WasmBinaryParse where Self: std::marker::Sized {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self>;
}

/// This is a trait for proxy types: types that wrap other types to change their parsing rules.
/// 
/// Proxy types are intended to wrap another type to change the parsing rules of the wrapped type.
/// 
/// Proxy types must implement the `WasmBinaryParse` trait as well as the `unwrap` methods of
/// `WasmBinaryParseProxy`.
/// The `unwrap` method returns the wrapped type.
/// 
/// # Example
/// 
/// The type `whasm::binary::Byte` is a proxy for an `u8`. It changes the parsing of an `u8` from
/// LEB-128 encoding, to parsing a raw byte from an iterator.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, Byte};
/// let mut iter = [0x8E].iter().copied();
/// let result = iter.parse::<Byte>().unwrap().unwrap();
/// assert_eq!(result, 0x8E);
/// 
/// let mut iter = [0x8E, 0x00].iter().copied();
/// let result = iter.parse::<u8>().unwrap();
/// assert_eq!(result, 0x0E);
/// ```
/// 
/// A proxy that discards the first byte before the encoded element.
/// 
/// ```
/// # use whasm::binary::{WasmBinary, WasmBinaryParse, WasmBinaryParseProxy, Result, Byte};
/// struct SkipOneByte<T> ( T );
/// impl<T: WasmBinaryParse> WasmBinaryParse for SkipOneByte<T> {
///     fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
///         let _: Byte = bin.parse()?;
///         Ok(Self(bin.parse()?))
///     }
/// }
/// impl<T: WasmBinaryParse> WasmBinaryParseProxy for SkipOneByte<T> {
///     type Inner = T;
///     fn unwrap(self) -> Self::Inner {
///         self.0
///     }
/// }
/// 
/// let mut iter = [0x00, 0x2A].iter().copied();
/// let result: SkipOneByte<Byte> = iter.parse().unwrap();
/// assert_eq!(result.unwrap(), 42)
/// ```
pub trait WasmBinaryParseProxy : WasmBinaryParse {
    type Inner;
    fn unwrap(self) -> Self::Inner;
}