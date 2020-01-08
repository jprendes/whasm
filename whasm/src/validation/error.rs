use err_derive::Error;

// TODO: Add real meaningful errors here.

#[derive(Debug, Error)]
pub enum Error {
    #[error(display = "Unexpected end of file.")]
    UnexpectedEndOfFile,
}