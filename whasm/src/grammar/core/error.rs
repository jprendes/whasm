//use failure::Error;
use err_derive::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Invalid variant discriminant `{}` while deserializing enum `{}`.", discriminant, ident)]
    InvalidEnumDiscriminant {
        ident: String,
        discriminant: String,
    },

    #[error(display = "Reading out of range signed integer.")]
    OutOfRangeSignedInteger,

    #[error(display = "Reading out of range signed integer.")]
    OutOfRangeUnsignedInteger,

    #[error(display = "Unexpected end of stream.")]
    UnexpectedEndOfStream,

    #[error(display = "Size of `{}` is smaller than expected.", ident)]
    RemainingBytesInStream {
        ident: String,
    },

    #[error(display = "Found unexpected `{}` instruction.", ident)]
    UnexpectedInstruction {
        ident: String,
    },

    #[error(display = "Unsatisfied match. Expected value `{}`, but found `{}`.", expected, actual)]
    UnsatisfiedMatch {
        expected: String,
        actual: String,
    },
}