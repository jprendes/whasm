//! This module defines the deserialization of arrays of with 8 or less elements.
//! 
//! A serialized array of size `N` will deserialize `N` elements of the same type.
//! The array elements must implement the `Grammar` trait.
//! An encoded array differs from an encoded `Vec<_>` in that the array does not encode its size,
//! since the size is known in advance.
//! 
//! # Example
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: [Byte; 4] = deserialize(&mut iter).unwrap();
//! assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
//! ```

use super::*;

impl<T: Grammar> Grammar for [T; 0] {
    fn deserialize<Iter: Iterator<Item=u8>>(_iter: &mut Iter) -> Result<Self> {
        Ok([])
    }
}

impl<T: Grammar> Grammar for [T; 1] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([deserialize(iter)?])
    }
}

impl<T: Grammar> Grammar for [T; 2] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

impl<T: Grammar> Grammar for [T; 3] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

impl<T: Grammar> Grammar for [T; 4] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

impl<T: Grammar> Grammar for [T; 5] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

impl<T: Grammar> Grammar for [T; 6] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

impl<T: Grammar> Grammar for [T; 7] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

impl<T: Grammar> Grammar for [T; 8] {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok([
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
            deserialize(iter)?,
        ])
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    #[test]
    fn can_deserialize_empty_array() {
        let mut iter = [0; 0].iter().copied();
        let result: [Byte; 0] = deserialize(&mut iter).unwrap();
        assert_eq!(result, [0; 0]);
    }

    #[test]
    fn can_deserialize_4_element_array() {
        let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
        let result: [Byte; 4] = deserialize(&mut iter).unwrap();
        assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
    }
}