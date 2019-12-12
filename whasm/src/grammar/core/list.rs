//! This module defines the deserialization of `whasm::grammar::List`.
//! 
//! A `List` is similar to a `Vec<_>`, except that the encoding of `Vec<_>` specifies the total
//! number of elements and that many elements are deserialized, while a `List` does not encode the
//! total number of elements and deserializes elements until the input iterator is exhausted.
//! 
//! # Example
//! 
//! ```
//! # use whasm::grammar::*;
//! let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
//! let result: List<Byte> = deserialize(&mut iter).unwrap();
//! assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
//! ```

use super::*;

/// A `List<T>` is a wrapper for a `Vec<T>`. It differs from a `Vec<T>` on how it is deserialized.
/// The serialization of a `Vec<T>` begins with the number of elements `N` in the vector followed by
/// each of the `N` elements.
/// The serialization of a `List<T>` starts with the first element and continues reading until the
/// iterator is exhausted.
/// 
/// After deserializing the last element of `List<T>`, the iterator must be exhausted. If not, the
/// `List<T>` will try to deserialize a new `T` and append it to the list.
/// 
/// # Example
/// 
/// ```
/// # use whasm::grammar::*;
/// let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
/// let result: List<Byte> = deserialize(&mut iter).unwrap();
/// assert_eq!(result, [0x01, 0x02, 0x03, 0x04]);
/// assert_eq!(iter.next(), None);
/// ```
/// 
/// Deserialization will return an error if the iterator is not exhausted.
/// 
/// ```
/// # use whasm::grammar::*;
/// let mut iter = [0x00, 0x00, 0x00, 0x00, 0x05, 0x06].iter().copied();
/// let result: Result<List<f32>> = deserialize(&mut iter);
/// assert!(result.is_err());
/// ```
#[derive(Debug, PartialEq)]
pub struct List<T: Grammar>(pub Vec<T>);

impl<T: Grammar> Grammar for List<T> {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok(List({
            let mut result = vec![];
            let mut iter = iter.peekable();
            while let Some(_) = iter.peek() {
                result.push(deserialize(&mut iter)?);
            }
            result
        }))
    }
}

impl<A, B> std::cmp::PartialEq<B> for List<A>
where A: Grammar, Vec<A>: PartialEq<B> {
    fn eq(&self, other: &B) -> bool {
        self.0 == *other
    }
}

impl<T: Grammar> Into<Vec<T>> for List<T> {
    fn into(self) -> Vec<T> { self.0 }
}

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

    #[test]
    fn can_deserialize_list() {
        let mut iter = [0x2A, 0xAA, 0x00, 0xAA, 0x80, 0x00].iter().copied();
        let result: List<u8> = deserialize(&mut iter).unwrap();
        assert_eq!(result, [42, 42, 42]);
    }
}