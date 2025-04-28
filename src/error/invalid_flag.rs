use crate::{flag::FlagKind, help::{Docs, FlagUsage}};

use super::{Error, Msg, InvalidArg, InvalidArgMsg};

#[derive(Debug, Clone, Copy)]
pub enum InvalidFlag
{
    FileAlreadySpecified,
    HelpAfterInteger
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
            Self::FileAlreadySpecified => FlagKind::File,
            Self::HelpAfterInteger => FlagKind::Help
        }
    }

    pub fn msg<'a>(&'a self, exe: &'static str, no: usize, arg: &str) -> Msg<'a>
    {
        match self
        {
            InvalidFlag::FileAlreadySpecified => Msg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some("You've already specified a filename.".into()),
                docs: Some(Docs::FlagUsage(FlagUsage {
                    exe,
                    flag: FlagKind::File
                }))
            },
            InvalidFlag::HelpAfterInteger => Msg {
                msg: InvalidArgMsg::Invalid.msg(no, arg),
                error: None,
                line: None,
                hint: Some(format!("You can't use{arg} after integer arguments.").into_boxed_str()),
                docs: Some(Docs::FlagUsage(FlagUsage {
                    exe,
                    flag: FlagKind::Help
                }))
            }
        }
    }
}