use core::fmt::Display;

use crate::{flag::Flag, help::FlagExample};

pub struct FlagsExamples;

impl Display for FlagsExamples
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
            let flag_example = FlagExample(flag);
            write!(f, "\n\t{flag_example}")?;
        }
    
        Ok(())
    }
}