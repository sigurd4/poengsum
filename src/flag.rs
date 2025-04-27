use core::{fmt::Display, ops::Deref};
use std::path::PathBuf;

use crate::error::InvalidArg;

#[derive(Clone, Copy, Debug)]
pub enum FlagOption<S>
where
    S: Deref<Target = str>
{
    Long(S),
    Short(char)
}
impl From<FlagOption<&str>> for FlagOption<Box<str>>
{
    fn from(opt: FlagOption<&str>) -> Self
    {
        opt.into()
    }
}
impl<S> FlagOption<S>
where
    S: Deref<Target = str>
{
    pub fn into<T>(self) -> FlagOption<T>
    where
        T: Deref<Target = str>,
        S: Into<T>
    {
        match self
        {
            Self::Long(opt) => FlagOption::Long(opt.into()),
            Self::Short(opt) => FlagOption::Short(opt)
        }
    }
}

impl<S> Display for FlagOption<S>
where
    S: Deref<Target = str>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            FlagOption::Long(opt) => opt.fmt(f),
            FlagOption::Short(opt) => opt.fmt(f),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FlagKind
{
    File
}

impl FlagKind
{
    pub const fn option(self) -> &'static str
    {
        match self
        {
            Self::File => "file"
        }
    }
    pub fn option_short(self) -> char
    {
        self.option().chars().next().unwrap()
    }
    pub fn args_example(self) -> Vec<&'static str>
    {
        match self
        {
            Self::File => vec!["<path>"]
        }
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

pub enum Flag
{
    File(PathBuf)
}

impl Flag
{
    pub const VARIANT_COUNT: usize = 1;
    pub const VARIANTS: [FlagKind; Self::VARIANT_COUNT] = [FlagKind::File];
}