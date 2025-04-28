use super::{Error, InvalidCall};

#[derive(Debug)]
pub enum ExpectedArg
{
    Filename
}
impl ExpectedArg
{
    pub fn at(self, exe: &'static str, no: usize) -> Error
    {
        Error::InvalidCall {
            exe,
            no,
            error: InvalidCall::ExpectedArg {
                error: self
            }
        }
    }
}