use crate::flag::FlagKind;

use super::{Error, InvalidArg};

#[derive(Debug, Clone, Copy)]
pub enum InvalidFlag
{
    FileAlreadySpecified
}

impl From<InvalidFlag> for InvalidArg
{
    fn from(error: InvalidFlag) -> Self
    {
        InvalidArg::InvalidFlag { error }
    }
}

impl InvalidFlag
{
    pub fn at(self, exe: &'static str, no: usize, arg: Option<Box<str>>) -> Error
    {
        InvalidArg::from(self).at(exe, no, arg)
    }

    pub fn related_flag(&self) -> FlagKind
    {
        match self
        {
            Self::FileAlreadySpecified => FlagKind::File
        }
    }
}