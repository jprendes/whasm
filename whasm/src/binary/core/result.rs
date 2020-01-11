//! This module defines the result type returned used by the parsing of WebAssembly binary file.

use crate::binary::Error;

pub type Result<T> = std::result::Result<T, Error>;