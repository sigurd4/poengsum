use core::{ num::ParseIntError, str::FromStr};

use crate::{flag::{FlagKind, FlagOption}, help::{ArgUsage, Docs, FlagUsage, FlagsUsages}};

use super::{Error, ErrorMsg, InvalidCall, InvalidFlag};

#[derive(Debug)]
pub enum InvalidArg
{
    UseTwoDots,
    CannotParseInteger {
        error: <usize as FromStr>::Err
    },
    NotInOrder {
        start: usize,
        end: usize
    },
    RoundZero,
    NonexistentFlag {
        flag: FlagOption<Box<str>>
    },
    InvalidFlag {
        error: InvalidFlag
    },
    IntegerAfterHelp
}

impl From<ParseIntError> for InvalidArg
{
    fn from(error: <usize as FromStr>::Err) -> Self
    {
        Self::CannotParseInteger {
            error
        }
    }
}

pub(super) enum InvalidArgMsg
{
    Parse,
    Invalid
}

impl InvalidArgMsg
{
    pub(super) fn msg(self, no: usize, arg: &str) -> Box<str>
    {
        let nth = Error::nth(no);
        match self
        {
            Self::Parse => format!("Unable to parse the {nth} commandline argument{arg}.").into_boxed_str(),
            Self::Invalid => format!("The {nth} argument{arg} is invalid.").into_boxed_str()
        }
    }
}

impl InvalidArg
{
    pub fn at(self, exe: &'static str, no: usize, arg: Option<Box<str>>) -> Error
    {
        Error::InvalidCall {
            exe,
            no,
            error: InvalidCall::InvalidArg {
                arg,
                error: self
            }
        }
    }

    pub fn msg<'a>(&'a self, exe: &'static str, no: usize, arg: &str) -> ErrorMsg<'a>
    {
        match self
        {
            InvalidArg::UseTwoDots => ErrorMsg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some("Did you mean to write '..' instead of '.'?".into()),
                docs: Some(Docs::ArgUsage(ArgUsage {
                    exe
                }))
            },
            InvalidArg::CannotParseInteger { error } => ErrorMsg {
                msg: InvalidArgMsg::Parse.msg(no, arg),
                error: Some(error),
                line: None,
                hint: Some("Round must be a valid number (a nonzero integer).".into()),
                docs: Some(Docs::ArgUsage(ArgUsage {
                    exe
                }))
            },
            InvalidArg::NotInOrder { start, end } => ErrorMsg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some(format!("{start} is larger than {end}, but this is not supported.").into_boxed_str()),
                docs: Some(Docs::ArgUsage(ArgUsage {
                    exe
                }))
            },
            InvalidArg::RoundZero => ErrorMsg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some("0 is not a valid round! Rounds start at 1, not 0.".into()),
                docs: Some(Docs::ArgUsage(ArgUsage {
                    exe
                }))
            },
            InvalidArg::NonexistentFlag { flag } => ErrorMsg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some(format!("There is no available option with the name \"{flag}\".").into_boxed_str()),
                docs: Some(Docs::FlagsUsages(FlagsUsages {
                    exe
                }))
            },
            InvalidArg::InvalidFlag { error } => error.msg(exe, no, arg),
            InvalidArg::IntegerAfterHelp => ErrorMsg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some(format!("Didn't expect argument{arg} after \"--help\".").into_boxed_str()),
                docs: Some(Docs::FlagUsage(FlagUsage {
                    exe,
                    flag: FlagKind::Help
                }))
            }
        }
    }
}