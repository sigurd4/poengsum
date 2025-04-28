use core::fmt::Display;

use colored::Colorize;

pub struct Example
{
    pub exe: &'static str,
    pub args: Vec<Box<str>>,
    pub effect: Option<Box<str>>
}

impl Display for Example
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

        if let Some(effect) = self.effect.as_ref()
        {
            let effect = effect.italic().bright_black();
            write!(f, " \t\t{effect}")?;
        }

        Ok(())
    }
}