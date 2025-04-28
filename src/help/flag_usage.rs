use core::fmt::Display;

use crate::flag::FlagKind;

pub struct FlagUsage<'a>
{
    pub exe: &'a str,
    pub flag: FlagKind
}

impl<'a> Display for FlagUsage<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let opt = self.flag.option();
        let opt_short = self.flag.option_short();
        let examples = self.flag.examples(self.exe);
        
        write!(f, "--{opt} or -{opt_short}")?;

        for example in examples
        {
            write!(f, "\n\t{example}")?;
        }
        Ok(())
    }
}