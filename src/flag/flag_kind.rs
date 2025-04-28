use core::{fmt::Display, ops::Deref};

use crate::{error::InvalidArg, help::Example};

use super::{Flag, FlagOption};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagKind
{
    Help,
    File
}

impl FlagKind
{
    pub const fn option(self) -> &'static str
    {
        match self
        {
            Self::Help => "help",
            Self::File => "file",
        }
    }
    pub fn option_short(self) -> char
    {
        self.option().chars().next().unwrap()
    }
    pub fn examples<'a>(self, exe: &'a str) -> Vec<Example<'a>>
    {
        match self
        {
            Self::Help => vec![
                Example {
                    exe,
                    args: vec!["--help"],
                    effect: Some("Shows usage of this program.")
                },
                Example {
                    exe,
                    args: vec!["--help", "--file"],
                    effect: Some("Shows usage of the option \"--file\".")
                }
            ],
            Self::File => vec![
                Example {
                    exe,
                    args: vec!["--file", "<path>"],
                    effect: Some("Loads a different poengsum-file.")
                }
            ]
        }
    }
}

impl Display for FlagKind
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.option().fmt(f)
    }
}

impl<S> TryFrom<FlagOption<S>> for FlagKind
where
    S: Deref<Target = str> + Into<Box<str>>
{
    type Error = InvalidArg;

    fn try_from(opt: FlagOption<S>) -> Result<Self, Self::Error>
    {
        for flag in Flag::VARIANTS
        {
            if match &opt
            {
                FlagOption::Long(opt) => flag.option() == &**opt,
                FlagOption::Short(opt) => flag.option_short() == *opt
            }
            {
                return Ok(flag)
            }
        }

        Err(InvalidArg::NonexistentFlag { flag: opt.into() })
    }
}