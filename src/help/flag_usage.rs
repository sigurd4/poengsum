use core::fmt::Display;

use colored::Colorize;

use crate::flag::FlagKind;

pub struct FlagUsage
{
    pub exe: &'static str,
    pub flag: FlagKind
}

impl Display for FlagUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let opt = self.flag.option();
        let opt_short = self.flag.option_short();
        let examples = self.flag.examples(self.exe);
        
        write!(f, "{header}", header = format!("Usage of --{opt} or -{opt_short}:").bold())?;

        for example in examples
        {
            write!(f, "\n\t{example}")?;
        }
        Ok(())
    }
}