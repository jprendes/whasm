//! This module defines the deserialization of `Vec<_>` and `VecVec<_>`.
//! 
//! A `Vec<_>` is encoded specifying the total number `N` of elements as `u32`, followed by `N`
//! encoded elements. To deserialize a `Vec<_>` the number of elements is deserialized and then
//! that many elements are deserialized and inserted in the vector.
//! 
//! A `VecVec<_>` is similar to a `Vec<_>`, but it differs in its encoding. A `VecVec<_>` is
//! encoded specifying a number `N` of sub-entries as u32, followed by `N` sub entries. Each sub
//! entry is encoded as a number `M` and an element. The number `M` is the number of times the
//! element in the sub entry is to be repeated.
//! 
//! # Example
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x04, 0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: Vec<Byte> = deserialize(&mut iter).unwrap();
//! assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
//! ```
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x02, 0x02, 0x2A, 0x03, 0x8E, 0x01].iter().copied();
//! let result: VecVec<u8> = deserialize(&mut iter).unwrap();
//! assert_eq!(result, vec![42, 42, 142, 142, 142]);
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

#[derive(Debug, PartialEq)]
pub struct VecVec<T: Grammar + Copy>(pub Vec<T>);

impl<T: Grammar + Copy> Grammar for VecVec<T> {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok(VecVec({
            let mut result = vec![];
            let size: u32 = deserialize(iter)?;
            for _ in 0..size {
                let size: u32 = deserialize(iter)?;
                let elem: T = deserialize(iter)?;
                for _ in 0..size {
                    result.push(elem);
                }
            }
            result
        }))
    }
}

impl<A, B> std::cmp::PartialEq<B> for VecVec<A>
where A: Grammar + Copy, Vec<A>: PartialEq<B> {
    fn eq(&self, other: &B) -> bool {
        self.0 == *other
    }
}

impl<T: Grammar + Copy> Into<Vec<T>> for VecVec<T> {
    fn into(self) -> Vec<T> { self.0 }
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

    #[test]
    fn can_deserialize_vec_vec() {
        let mut iter = [0x02, 0x02, 0x2A, 0x03, 0x8E, 0x01].iter().copied();
        let result: VecVec<u8> = deserialize(&mut iter).unwrap();
        assert_eq!(result, vec![42, 42, 142, 142, 142]);
    }
}