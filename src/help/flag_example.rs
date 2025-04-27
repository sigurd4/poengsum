use core::fmt::Display;

use crate::flag::FlagKind;

pub struct FlagExample(pub FlagKind);

impl Display for FlagExample
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let opt = self.0.option();
        let opt_short = self.0.option_short();
        let opt_args = self.0.args_example();
        
        write!(f, "--{opt} or -{opt_short}")?;

        for arg in opt_args
        {
            write!(f, " {arg}")?;
        }
        Ok(())
    }
}