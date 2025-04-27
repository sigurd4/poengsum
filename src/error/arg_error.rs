use super::{Error, ExpectedArg, InvalidArg, InvalidFlag};

pub enum ArgError
{
    InvalidArg(InvalidArg),
    ExpectedArg(ExpectedArg)
}

impl From<InvalidArg> for ArgError
{
    fn from(error: InvalidArg) -> Self
    {
        Self::InvalidArg(error)
    }
}
impl From<InvalidFlag> for ArgError
{
    fn from(error: InvalidFlag) -> Self
    {
        InvalidArg::from(error).into()
    }
}
impl From<ExpectedArg> for ArgError
{
    fn from(error: ExpectedArg) -> Self
    {
        Self::ExpectedArg(error)
    }
}

impl ArgError
{
    pub fn at(self, no: usize, arg: Option<Box<str>>) -> Error
    {
        match self
        {
            ArgError::InvalidArg(err) => err.at(no, arg),
            ArgError::ExpectedArg(err) => err.at(no),
        }
    }
}