use super::*;

#[derive(Debug)]
#[derive(Clone, Copy, Hash)]
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Byte(pub u8);

impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}

impl Grammar for Byte {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self> {
        Ok(Byte(iter.next().ok_or(Error::UnexpectedEndOfStream)?))
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
    use crate::grammar::*;

    #[test]
    fn can_deserialize_byte() {
        let mut iter = [0x8E].iter().copied();
        let Byte(result) = deserialize(&mut iter).unwrap();
        assert_eq!(result, 0x8E);
    }
}