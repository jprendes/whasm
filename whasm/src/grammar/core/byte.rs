//! This module defines the deserialization of `whasm::grammar::Byte`.

use super::*;

/// Wrapper for a byte of information. It differs from an `u8` on how it is deserialized.
/// A `Byte` reads exactly one byte from the input iterator, while a `u8` could potentially read
/// several bytes (since unsigned integers are encoded using LEB-128).
/// 
/// A `Byte` takes exactly one byte of memory, and can be transmuted to `u8`.
/// This is specially useful since it allows transmuting a vector of `Byte`s to a vector of `u8`.
/// 
/// # Example
/// 
/// ```
/// # use whasm::grammar::*;
/// let mut iter = [0x8E].iter().copied();
/// let Byte(result) = deserialize(&mut iter).unwrap();
/// assert_eq!(result, 0x8E);
/// ```
/// 
/// Deserialization will return an error if the iterator is exhausted while deserializing.
/// 
/// ```
/// # use whasm::grammar::*;
/// let mut iter = [0x00].iter().copied();
/// let Byte(_) = deserialize(&mut iter).unwrap();
/// let result: Result<Byte> = deserialize(&mut iter);
/// assert!(result.is_err());
/// ```
#[derive(Clone, Copy, Hash)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Byte(pub u8);

impl Grammar for Byte {
    /// Read a `Byte` from an `Iterator<u8>`. It returns `Ok(Byte(_))` on success, and
    /// `Error(Error::UnexpectedEndOfStream)` if the iterator returns `None`.
    /// 
    /// Deserializing a `Byte` instead of using `iter.next()` is an easy way to convert the
    /// `Option<u8>` returned by the iterator into a `Result<u8>` with a meaningful error type.
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok(Byte(iter.next().ok_or(Error::UnexpectedEndOfStream)?))
    }
}

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

impl std::cmp::PartialEq<u8> for Byte {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialEq<char> for Byte {
    fn eq(&self, other: &char) -> bool {
        (self.0 as char) == *other
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    #[test]
    fn can_deserialize_byte() {
        let mut iter = [0x8E].iter().copied();
        let Byte(result) = deserialize(&mut iter).unwrap();
        assert_eq!(result, 0x8E);
    }

    #[test]
    fn byte_uses_exactly_1_byte() {
        assert_eq!(std::mem::size_of::<Byte>(), 1);
    }

    #[test]
    fn byte_transmutes_to_u8() {
        let byte = Byte(42);
        let transmuted: u8 = unsafe { std::mem::transmute(byte) };
        assert_eq!(transmuted, 42);
    }

    #[test]
    fn byte_vector_transmutes_to_u8_vector() {
        let byte = vec![Byte(42), Byte(0x33), Byte(0x55)];
        let transmuted: Vec<u8> = unsafe { std::mem::transmute(byte) };
        assert_eq!(transmuted, vec![42, 0x33, 0x55]);
    }
}
