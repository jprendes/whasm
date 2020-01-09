//! This module defines the parsing of `whasm::binary::core::sized::Sized`.
//! 
//! A `Sized` is a proxy type. Parsing a type using this proxy means that the encoded value is
//! preceded by an `u32` representing the size in bytes of the encoded value.
//! 
//! A helper type `Consume` is defined to consume all bytes available in the iterator. This is
//! useful to skip the parsing of a Sized element.
//! 
//! # Example
//! 
//! After parsing `unwrap` can be used to obtain the wrapped value.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, WasmBinaryParseProxy, Byte, Result, Error};
//! # use whasm::binary::core::sized::Sized;
//! let mut iter = [0x02, 0x2A, 0x2B].iter().copied();
//! let result: [Byte; 2] = iter.parse::<Sized<_>>().unwrap().unwrap();
//! assert_eq!(result, [0x2A, 0x2B]);
//! ```
//! 
//! While parsing, this type will verify that the exact number of bytes are read from the iterator
//! and returns `Err(Error::SizeMissmatch)` if that is not the case.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, Byte, Result, Error};
//! # use whasm::binary::core::sized::Sized;
//! let mut iter = [0x02, 0x2A, 0x2B].iter().copied();
//! let result: Result<Sized<Byte>> = iter.parse();
//! assert_eq!(result, Err(Error::SizeMissmatch));
//! ```
//! 
//! The `Consume` type can be used to skip over the content of a Sized element.
//! 
//! ```
//! # use whasm::binary::{WasmBinary, Byte, Result, Error};
//! # use whasm::binary::core::sized::{Sized, Consume};
//! let mut iter = [0x02, 0x2A, 0x2B, 0x2C].iter().copied();
//! let _: Sized<Consume> = iter.parse().unwrap();
//! let Byte(result) = iter.parse().unwrap();
//! assert_eq!(result, 0x2C);
//! ```

use crate::binary::{WasmBinaryParse, WasmBinary, Result, Error, WasmBinaryParseProxy};

pub struct Sized<T> ( T );
impl<T: WasmBinaryParse> WasmBinaryParseProxy for Sized<T> {
    type Inner = T;

    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let size: u32 = bin.parse()?;
        let mut count = 0;
        let mut bin = bin
            .take(size as usize)
            .inspect(|_| count = count + 1);
        let result = bin.parse()?;
        if size != count {
            Err(Error::SizeMissmatch)
        } else {
            Ok(Self(result))
        }
    }

    fn unwrap(self) -> Self::Inner { self.0 }
}

impl<T: PartialEq> PartialEq<T> for Sized<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

impl<T: PartialEq> PartialEq<Sized<T>> for Sized<T> {
    fn eq(&self, other: &Sized<T>) -> bool {
        self.0 == other.0
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Sized<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_tuple("Sized").field(&self.0).finish()
    }
}

pub struct Consume { pub count: usize }
impl WasmBinaryParse for Consume {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Consume { count: bin.count() })
    }
}

#[cfg(test)]
mod test {
    use crate::binary::{WasmBinary, WasmBinaryParseProxy, Byte, Result, Error};
    use crate::binary::core::sized::{Sized, Consume};

    #[test]
    fn can_parse_sized_element() {
        let mut iter = [0x02, 0x01, 0x02].iter().copied();
        let result: [Byte; 2] = iter.parse::<Sized<_>>().unwrap().unwrap();
        assert_eq!(result, [0x01, 0x02]);
    }

    #[test]
    fn fails_to_parse_sized_element() {
        let mut iter = [0x03, 0x01, 0x02, 0x03].iter().copied();
        let result: Result<Sized<[Byte; 2]>> = iter.parse::<Sized<_>>();
        assert_eq!(result, Err(Error::SizeMissmatch));
    }

    #[test]
    fn can_parse_consume() {
        let mut iter = [0x02, 0x01, 0x02].iter().copied();
        let _: Sized<Consume> = iter.parse().unwrap();
    }
}