use std::{borrow::Cow, path::Path};

use super::{Error, InvalidIO, InvalidSyntax};

#[derive(Debug)]
pub enum InvalidRead
{
    InvalidIO {
        io_error: std::io::Error,
        error: InvalidIO
    },
    InvalidSyntax {
        error: InvalidSyntax
    },
}

impl InvalidRead
{
    pub fn at(self, file: Cow<'static, Path>) -> Error
    {
        Error::InvalidRead {
            file,
            error: self
        }
    }
}