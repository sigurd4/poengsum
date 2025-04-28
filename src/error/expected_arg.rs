use crate::{flag::FlagKind, help::{Docs, FlagUsage}};

use super::{Error, Msg, InvalidCall};

#[derive(Debug)]
pub enum ExpectedArg
{
    Filename
}

struct ExpectedArgMsg;

impl ExpectedArgMsg
{
    fn msg(self, no: usize, what: &str) -> Box<str>
    {
        let nth = Error::nth(no);
        format!("Expected {what} as the {nth} argument.").into_boxed_str()
    }
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

    pub fn msg(&self, exe: &'static str, no: usize) -> Msg
    {
        match self
        {
            ExpectedArg::Filename => Msg {
                msg: ExpectedArgMsg.msg(no, "a filename"),
                error: None,
                line: None,
                hint: None,
                docs: Some(Docs::FlagUsage(FlagUsage {
                    exe,
                    flag: FlagKind::File
                }))
            },
        }
    }
}