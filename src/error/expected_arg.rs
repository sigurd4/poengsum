use super::{Error, InvalidCall};

#[derive(Debug)]
pub enum ExpectedArg
{
    Filename
}
impl ExpectedArg
{
    pub fn at(self, no: usize) -> Error
    {
        Error::InvalidCall {
            no,
            error: InvalidCall::ExpectedArg {
                error: self
            }
        }
    }
}