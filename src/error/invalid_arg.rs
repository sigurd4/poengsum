use core::{ num::ParseIntError, str::FromStr};

use crate::flag::FlagOption;

use super::{Error, InvalidCall, InvalidFlag};

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
}