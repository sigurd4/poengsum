use core::fmt::Display;

use colored::Colorize;

use crate::{flag::Flag, help::FlagUsage};

pub struct FlagsUsages
{
    pub exe: &'static str
}

impl Display for FlagsUsages
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if Flag::VARIANT_COUNT == 0
        {
            return Ok(())
        }
        write!(f, "{header}", header = "Available options are:".bold())?;
    
        for flag in Flag::VARIANTS
        {
            write!(f, "\n")?;

            let flag_example = FlagUsage {
                exe: self.exe,
                flag
            };

            let opt = flag_example.flag.option();
            let opt_short = flag_example.flag.option_short();
            let examples = flag_example.flag.examples(self.exe);
            
            let of = format!("--{opt} or -{opt_short}:").italic().bright_black();
            write!(f, "{of}")?;
    
            for example in examples
            {
                write!(f, "\n\t{example}")?;
            }
        }
    
        Ok(())
    }
}