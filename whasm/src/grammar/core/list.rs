use super::*;

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

#[cfg(test)]
mod test {
    use crate::grammar::*;

    #[test]
    fn can_deserialize_list() {
        let mut iter = [0x2A, 0xAA, 0x00, 0xAA, 0x80, 0x00].iter().copied();
        let List(result): List<u8> = deserialize(&mut iter).unwrap();
        assert_eq!(result, [42, 42, 42]);
    }
}