pub mod core;
pub mod error;

pub mod idx;
pub mod instr;
pub mod module;
pub mod ty;

pub use self::core::byte::Byte;
pub use self::error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait Parse<T> {
    fn parse(&mut self) -> Result<T>;
}

pub trait WasmBinary: Iterator<Item=u8>
where Self: std::marker::Sized {
    fn parse<T: WasmBinaryParse>(&mut self) -> Result<T> {
        <T as WasmBinaryParse>::parse(self)
    }
}

impl<T> WasmBinary for T
where T: Iterator<Item=u8> {
}

pub trait WasmBinaryParse where Self: std::marker::Sized {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self>;
}