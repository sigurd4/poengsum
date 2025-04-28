use core::fmt::Display;

use crate::{flag::Flag, help::FlagUsage};

pub struct FlagsUsages<'a>
{
    pub exe: &'a str
}

impl<'a> Display for FlagsUsages<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if Flag::VARIANT_COUNT == 0
        {
            return Ok(())
        }
        write!(f, "Available options are:")?;
    
        for flag in Flag::VARIANTS
        {
            let flag_example = FlagUsage {
                exe: self.exe,
                flag
            };
            write!(f, "\n\t{flag_example}")?;
        }
    
        Ok(())
    }
}