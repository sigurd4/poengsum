use core::fmt::Display;
use std::{borrow::Cow, path::Path};

use crate::help::Help;

moddef::moddef!(
    flat(pub) mod {
        arg_error,
        expected_arg,
        insufficient_data,
        invalid_arg,
        invalid_call,
        invalid_flag,
        invalid_io,
        invalid_read,
        invalid_syntax,
        msg,
    }
);

#[derive(thiserror::Error, Debug)]
pub enum Error
{
    InvalidRead {
        file: Cow<'static, Path>,
        error: InvalidRead
    },
    InvalidCall {
        exe: &'static str,
        no: usize,
        error: InvalidCall
    },
    InsufficientData {
        error: InsufficientData
    },
    NoExecutable,
    ShowHelp {
        help: Help
    }
}

impl Error
{
    fn nth(no: usize) -> Box<str>
    {
        let suffix = match no
        {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };
        format!("{no}{suffix}").into_boxed_str()
    }

    fn msg(&self) -> Msg<'_>
    {
        match self
        {
            Error::InvalidRead { file, error } => error.msg(file),
            Error::InvalidCall { exe, no, error } => error.msg(exe, *no),
            Error::InsufficientData { error } => error.msg(),
            Error::NoExecutable => Msg {
                msg: "You somehow managed to run this binary without even a 0th argument.".into(),
                error: None,
                line: None,
                hint: Some("Not sure how you did it, but don't do that.".into()),
                docs: None
            },
            Error::ShowHelp { help } => help.msg(),
        }
    }
}

impl Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.msg().fmt(f)
    }
}