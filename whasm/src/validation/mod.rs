pub mod context;
pub mod error;
pub mod stacks;

pub mod idx;
pub mod instr;
pub mod module;
pub mod ty;

pub use self::context::Context;
pub use self::error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait ValidationEntry<'a>
where Self: Sized {
    type ValidationResult;
    fn validate(&'a self) -> Result<Self::ValidationResult>;
}

pub trait Validate<'a>
where Self: Sized {
    type ValidationResult;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult>;
}