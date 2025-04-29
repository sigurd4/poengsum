use core::fmt::Display;

use crate::{flag::Flag, help::FlagUsage, style};

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
        write!(f, "{header}", header = style::header("Available options are:"))?;
    
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
            
            let of = style::info(format!("--{opt} or -{opt_short}:"));
            write!(f, "{of}")?;
    
            for example in examples
            {
                write!(f, "\n\t{example}")?;
            }
        }
    
        Ok(())
    }
}