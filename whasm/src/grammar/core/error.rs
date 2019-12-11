//! This module provides an `enum`, with variants for each type of error produced by `whasm::grammar`.

use err_derive::Error;

/// This `enum` contains variants for each type of error produced by the different components of
/// `whasm::grammar`.
/// The `enum` implements the `std::error::Error` trait, so it can be stored in a
/// `Box<dyn std::error::Error>`.
/// 
/// ```
/// # use whasm::grammar::core::error::Error;
/// fn may_error() -> std::result::Result<(), Box<dyn std::error::Error>> {
///     Err(Error::UnexpectedEndOfStream)?
/// }
/// ```
#[derive(Debug, PartialEq, Error)]
pub enum Error {
    /// Error produced when deserializing an `enum`, and the discriminant does not match any
    /// variant.
    #[error(display = "Invalid variant discriminant `{}` while deserializing enum `{}`.", discriminant, ident)]
    InvalidEnumDiscriminant {
        ident: String,
        discriminant: String,
    },

    /// Error produced when deserializing a signed integer, the encoded value can not be
    /// represented for the corresponding type.
    #[error(display = "Reading out of range signed integer.")]
    OutOfRangeSignedInteger,

    /// Error produced when deserializing an unsigned integer, the encoded value can not be
    /// represented for the corresponding type.
    #[error(display = "Reading out of range signed integer.")]
    OutOfRangeUnsignedInteger,

    /// Error produced when deserializing would require reading past the end of the input iterator.
    #[error(display = "Unexpected end of stream.")]
    UnexpectedEndOfStream,

    /// Error produced when deserializing a sized `struct` where the deserialization consumes less
    /// bytes than expected.
    #[error(display = "Size of `{}` is smaller than expected.", ident)]
    RemainingBytesInStream {
        ident: String,
    },

    /// Error produced when an instruction variant is deserialized in a context where it is not
    /// expected (e.g., deserializing an `Else` outside of an `If` block).
    #[error(display = "Found unexpected `{}` instruction.", ident)]
    UnexpectedInstruction {
        ident: String,
    },

    /// Error produced when deserializing a field of a struct with a `matching` attribute that is
    /// not satisfied.
    #[error(display = "Unsatisfied match. Expected value `{}`, but found `{}`.", expected, actual)]
    UnsatisfiedMatch {
        expected: String,
        actual: String,
    },
}