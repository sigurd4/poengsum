use super::{ExpectedArg, InvalidArg};

#[derive(Debug)]
pub enum InvalidCall
{
    InvalidArg {
        arg: Option<Box<str>>,
        error: InvalidArg
    },
    ExpectedArg {
        error: ExpectedArg
    }
}