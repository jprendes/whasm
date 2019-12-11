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