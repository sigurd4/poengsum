use super::{Msg, ExpectedArg, InvalidArg};

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

impl InvalidCall
{
    pub fn msg(&self, exe: &'static str, no: usize) -> Msg<'_>
    {
        match self
        {
            InvalidCall::InvalidArg { arg, error } => {
                let arg = arg.as_ref()
                    .map(|arg| format!(" \"{arg}\""))
                    .unwrap_or_else(String::new);
                error.msg(exe, no, &arg)
            }
            InvalidCall::ExpectedArg { error } => error.msg(exe, no),
        }
    }
}