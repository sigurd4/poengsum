use core::fmt::Display;

use colored::Colorize;

use super::LineExample;

pub struct Rules;

impl Display for Rules
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let examples = [
            LineExample {
                team: "Quizzbuzz".into(),
                points: "3 5 15".into(),
                effect: Some("A team that got 3 points in the first round, then 5, and then 15 in the final.".into())
            }
        ];

        if !examples.is_empty()
        {
            writeln!(f, "{header}", header = "Example:".bold())?;
            for example in examples
            {
                writeln!(f, "\t{example}")?;
            }
            writeln!(f, "")?;
        }

        write!(f, "{rules}", rules = "One team on each line, followed by a ':', then the team's points for each round separated by whitespace.".italic().bright_black())
    }
}