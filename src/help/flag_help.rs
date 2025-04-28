use core::fmt::Display;

use crate::flag::FlagKind;

use super::FlagUsage;

pub struct FlagHelp
{
    pub exe: &'static str,
    pub flag: FlagKind,
    pub extra: Option<&'static str>
}

impl Display for FlagHelp
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let exe = self.exe;
        let default_file = crate::default_file_path().display();
        match self.flag
        {
            FlagKind::Help => writeln!(f,
                "The flag \"--help\" can be used to display instructions on how to use \"{exe}\", and with various different flags.\n\"--help\" alone will display general usage of the command.\n\"--help\" followed by another flag will tell you about the usage of that flag."
            )?,
            FlagKind::File => writeln!(f,
                "By default, the file that the score is read from is \"{default_file}\", but you can use a different file by setting the \"--file\" flag, followed by a path."
            )?
        }
        if let Some(extra) = self.extra
        {
            writeln!(f, "{extra}")?;
        }

        let example = FlagUsage {
            exe,
            flag: self.flag
        };

        write!(f, "\nUsage of {example}")?;

        Ok(())
    }
}