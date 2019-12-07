pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Grammar
where Self: Sized {
    fn deserialize<Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<Self>;
}

pub fn deserialize<T: Grammar, Iter: Iterator<Item=u8>>(iter: &mut Iter) -> Result<T> {
    <T as Grammar>::deserialize(iter)
}

pub mod unsigned;
pub mod signed;
pub mod byte;
pub mod vec;
pub mod string;
pub mod float;
pub mod array;
pub mod error;
pub mod list;
pub mod derive;

pub use self::byte::Byte;
pub use self::list::List;
pub use self::error::Error;
pub use self::derive::Grammar;