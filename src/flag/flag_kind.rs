use core::{fmt::Display, ops::Deref};

use crate::{error::InvalidArg, help::CallExample};

use super::{Flag, FlagOption};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagKind
{
    Help,
    File,
    Rev
}

impl FlagKind
{
    pub const fn option(self) -> &'static str
    {
        match self
        {
            Self::Help => "help",
            Self::File => "file",
            Self::Rev => "rev"
        }
    }
    pub fn option_short(self) -> char
    {
        self.option().chars().next().unwrap()
    }
    pub fn help(self, exe: &'static str) -> String
    {
        match self
        {
            FlagKind::Help => format!(
                "The flag \"--help\" can be used to display instructions on how to use \"{exe}\", and with various different flags."
            ),
            FlagKind::File => {
                let default_file = crate::default_file_path().display();
                format!(
                    "By default, the file that the score is read from is \"{default_file}\", but you can use a different file by setting the \"--file\" flag, followed by a path."
                )
            },
            FlagKind::Rev => "Reverses the direction in which the teams' score is shown in the scoreboard.".into()
        }
    }
    pub fn examples(self, exe: &'static str) -> Vec<CallExample>
    {
        match self
        {
            Self::Help => core::iter::once(CallExample {
                exe,
                args: vec!["--help".into()],
                effect: Some("Shows usage of this program.".into())
            }).chain(Flag::VARIANTS.into_iter()
                .filter_map(|flag| if flag != FlagKind::Help
                {
                    Some(CallExample {
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
                CallExample {
                    exe,
                    args: vec!["--file".into(), crate::default_file_path().to_string_lossy().into()],
                    effect: Some("Loads a different poengsum-file.".into())
                }
            ],
            Self::Rev => vec![
                CallExample {
                    exe,
                    args: vec!["--rev".into()],
                    effect: Some("Displays the leaderboard in reverse.".into())
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