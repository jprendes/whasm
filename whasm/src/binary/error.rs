use super::Byte;
use err_derive::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error(display = "Unexpected end of file.")]
    UnexpectedEndOfFile,

    #[error(display = "Unexpected variant id `{}` for type `{}`.", id, ty)]
    InvalidVariantId { id: u8, ty: &'static str },

    #[error(display = "Reading out of range signed integer.")]
    OutOfRangeSignedInteger,

    #[error(display = "Reading out of range signed integer.")]
    OutOfRangeUnsignedInteger,

    #[error(display = "Byte sequence is not a valid utf-8 encoding.")]
    InvalidUtf8Encoding,

    #[error(display = "Unexpected `{}` opcode.", instr)]
    UnexpectedOpcode { instr: &'static str },

    #[error(display = "Invalid magic number in preamble.")]
    InvalidPreambleMagic,

    #[error(display = "Invalid version number in preamble.")]
    InvalidPreambleVersion,

    #[error(display = "Unexpected section id `{}`.", id)]
    UnexpectedSectionId { id: Byte },

    #[error(display = "Invalid section id `{}`.", id)]
    InvalidSectionId { id: Byte },

    #[error(display = "The size of section with id `{}` does not match its content.", id)]
    SectionSizeMissmatch { id: Byte },

    #[error(display = "The size of the function does not match its content.")]
    FunctionSizeMissmatch,

    #[error(display = "The size of the element does not match its content.")]
    SizeMissmatch,
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::InvalidUtf8Encoding
    }
}