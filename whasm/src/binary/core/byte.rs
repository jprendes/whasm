use crate::binary::{WasmBinary, Result, Error, core::proxy::WasmBinaryParseProxy};

pub struct Byte(pub u8);

impl WasmBinaryParseProxy for Byte {
    type Inner = u8;

    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        match bin.next() {
            Some(byte) => Ok(Byte(byte)),
            None => Err(Error::UnexpectedEndOfFile)?,
        }
    }

    fn unwrap(self) -> Self::Inner { self.0 }
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

impl From<Byte> for u8 {
    fn from(byte: Byte) -> u8 { byte.0 }
}

impl From<Byte> for char {
    fn from(byte: Byte) -> char { byte.0.into() }
}