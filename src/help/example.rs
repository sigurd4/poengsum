use core::fmt::Display;

use colored::Colorize;

pub struct Example<'a>
{
    pub exe: &'a str,
    pub args: Vec<&'a str>,
    pub effect: Option<&'a str>
}

impl<'a> Display for Example<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let exe = self.exe.blue();
        write!(f, "{exe}")?;

        for arg in self.args.iter()
        {
            let arg = arg.cyan();
            write!(f, " {arg}")?;
        }

        if let Some(effect) = self.effect
        {
            let effect = effect.italic().bright_black();
            write!(f, " \t\t{effect}")?;
        }

        Ok(())
    }
}