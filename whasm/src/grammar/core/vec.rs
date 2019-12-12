//! This module defines the deserialization of `Vec<_>`.
//! 
//! A `Vec<_>` is encoded specifying the total number `N` of elements as `u32`, followed by `N`
//! encoded elements. To deserialize a `Vec<_>` the number of elements is deserialized and then
//! that many elements are deserialized and inserted in the vector.
//! 
//! # Example
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x04, 0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: Vec<Byte> = deserialize(&mut iter).unwrap();
//! assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
//! ```

use super::*;

impl<T: Grammar> Grammar for Vec<T> {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        let size: u32 = deserialize(iter)?;
        let mut result = Vec::with_capacity(size as usize);
        for _ in 0..size {
            result.push(deserialize(iter)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    #[test]
    fn can_deserialize_vec() {
        let mut iter = [0x04, 0x01, 0x02, 0x03, 0x04].iter().copied();
        let result: Vec<u8> = deserialize(&mut iter).unwrap();
        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04]);
    }
}