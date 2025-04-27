use core::{ num::ParseIntError, str::FromStr};

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
        flag: String
    },
    InvalidFlag {
        error: InvalidFlag
    }
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
    pub fn at(self, no: usize, arg: Option<String>) -> Error
    {
        Error::InvalidCall {
            no,
            error: InvalidCall::InvalidArg {
                arg,
                error: self
            }
        }
    }
}