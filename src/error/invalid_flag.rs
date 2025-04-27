use super::{Error, InvalidArg};

#[derive(Debug)]
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
    pub fn at(self, no: usize, arg: Option<Box<str>>) -> Error
    {
        InvalidArg::from(self).at(no, arg)
    }
}