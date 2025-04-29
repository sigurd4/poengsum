use core::fmt::Display;

use crate::style;

use super::CallExample;

pub struct ArgUsage {
    pub exe: &'static str
}

impl ArgUsage
{
    pub fn examples(&self) -> [CallExample; 4]
    {
        let Self { exe } = self;
        
        [
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
        ]
    }
}

impl Display for ArgUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let examples = self.examples();

        if examples.is_empty()
        {
            return Ok(())
        }

        write!(f, "{header}", header = style::header("Usage:"))?;

        for example in examples
        {
            write!(f, "\n\t{example}")?;
        }
    
        Ok(())
    }
}