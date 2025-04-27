use super::{ExpectedArg, InvalidArg};

#[derive(Debug)]
pub enum InvalidCall
{
    InvalidArg {
        arg: Option<String>,
        error: InvalidArg
    },
    ExpectedArg {
        error: ExpectedArg
    }
}