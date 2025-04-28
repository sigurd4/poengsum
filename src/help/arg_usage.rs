use core::fmt::Display;

use colored::Colorize;

use super::CallExample;

pub struct ArgUsage {
    pub exe: &'static str
}

impl Display for ArgUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self { exe } = self;
    
        let examples = [
            CallExample {
                exe,
                args: vec![],
                effect: Some("Shows total scoreboard for all rounds.".into())
            },
            CallExample {
                exe,
                args: vec!["2".into()],
                effect: Some("Shows scoreboard for round 2.".into())
            },
            CallExample {
                exe,
                args: vec!["1".into(), "3".into(), "5".into()],
                effect: Some("Shows the total scoreboard given an alternative sequence of rounds.".into())
            },
            CallExample {
                exe,
                args: vec!["1".into(), "3".into(), "5..7".into(), "9".into()],
                effect: Some("Ranges can be used to include a contiguous sequence of rounds.".into())
            }
        ];

        if examples.len() == 0
        {
            return Ok(())
        }

        write!(f, "{header}", header = "Usage:".bold())?;

        for example in examples
        {
            write!(f, "\n\t{example}")?;
        }
    
        Ok(())
    }
}