use core::fmt::Display;

use crate::{flag::FlagKind, style};

use super::FlagUsage;

pub struct FlagHelp
{
    pub exe: &'static str,
    pub flag: FlagKind
}

impl Display for FlagHelp
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self { exe, flag } = self;

        let usage = FlagUsage {
            exe,
            flag: *flag
        };

        let help = style::info(flag.help(exe));

        write!(f, "{usage}\n\n{help}")?;

        Ok(())
    }
}