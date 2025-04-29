use std::{io::ErrorKind, path::Path};

use crate::help::{Docs, Rules};

use super::{Msg, Severity, OffendingLine};

#[derive(Debug)]
pub enum InvalidIO
{
    Open,
    Read {
        row: usize
    }
}

impl InvalidIO
{
    pub fn msg<'a>(&'a self, file: &'a Path, io_error: &'a std::io::Error) -> Msg<'a>
    {
        let file_display = file.display();

        let hint = |placeholder: Option<Box<str>>| match io_error.kind()
        {
            ErrorKind::NotFound => if file == crate::default_file_path()
            {
                (
                    Some("If it doesn't exist, create it!".into()),
                    Some(Docs::Rules(Rules))
                )
            }
            else
            {
                (
                    None,
                    None
                )
            },
            ErrorKind::PermissionDenied => (
                Some("You don't have permission to view this file.".into()),
                None
            ),
            ErrorKind::IsADirectory => (
                Some(format!("There's, for some reason, a directory with the same name as {file_display}.\nThis program needs {file_display} to be a file, not a directory.").into_boxed_str()),
                None
            ),
            ErrorKind::Other => (
                placeholder,
                None
            ),
            _ => (
                None,
                None
            ),
        };

        match self
        {
            InvalidIO::Open => {
                let (hint, docs) = hint(None);
                Msg {
                    msg: format!("Failed to open file \"{file_display}\".").into_boxed_str(),
                    error: Some(io_error),
                    line: None,
                    hint,
                    docs
                }
            },
            InvalidIO::Read { row } => {
                let (hint, docs) = hint(None);
                Msg {
                    msg: format!("Failed to open file \"{file_display}\".").into_boxed_str(),
                    error: Some(io_error),
                    line: Some(OffendingLine {
                        file,
                        severity: Severity::Error,
                        line: None,
                        row: *row,
                        col: None
                    }),
                    hint,
                    docs
                }
            },
        }
    }
}