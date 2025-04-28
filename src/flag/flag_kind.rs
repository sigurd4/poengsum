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
    pub fn examples(self, exe: &'static str) -> Vec<Example>
    {
        match self
        {
            Self::Help => core::iter::once(Example {
                exe,
                args: vec!["--help".into()],
                effect: Some("Shows usage of this program.".into())
            }).chain(Flag::VARIANTS.into_iter()
                .filter_map(|flag| if flag != FlagKind::Help
                {
                    Some(Example {
                        exe,
                        args: vec!["--help".into(), format!("--{flag}").into_boxed_str()],
                        effect: Some(format!("Shows usage of the option \"--{flag}\".").into_boxed_str())
                    })
                }
                else
                {
                    None
                })
            ).collect(),
            Self::File => vec![
                Example {
                    exe,
                    args: vec!["--file".into(), "<path>".into()],
                    effect: Some("Loads a different poengsum-file.".into())
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