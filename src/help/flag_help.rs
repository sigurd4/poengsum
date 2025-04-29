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

        let default_file = crate::default_file_path().display();
        
        let help = style::info(
            match self.flag
            {
                FlagKind::Help => format!(
                    "The flag \"--help\" can be used to display instructions on how to use \"{exe}\", and with various different flags."
                ),
                FlagKind::File => format!(
                    "By default, the file that the score is read from is \"{default_file}\", but you can use a different file by setting the \"--file\" flag, followed by a path."
                )
            }
        );

        write!(f, "{usage}\n\n{help}")?;

        Ok(())
    }
}