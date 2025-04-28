use std::{borrow::Cow, path::Path};

use super::{Error, Msg, InvalidIO, InvalidSyntax};

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

    pub fn msg<'a>(&'a self, file: &'a Path) -> Msg<'a>
    {
        match self
        {
            InvalidRead::InvalidIO { io_error, error } => error.msg(file, io_error),
            InvalidRead::InvalidSyntax { error } => error.msg(file),
        }
    }
}