use core::fmt::Display;

use crate::style;

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
                effect: Some("A team that got 3 points in the first round, then 5, and then 15 in the final.".into()),
                row: 1
            }
        ];

        let default_file = crate::default_file_path().display();

        writeln!(f, "{rules}", rules = style::info(format!("\"poengsum\" looks for a poengsum-file (\"{default_file}\" by default) and displays a leaderboard for all the teams listed in it, given the points they got for each round.")))?;

        if !examples.is_empty()
        {
            writeln!(f, "\n{header}", header = style::header("Example:"))?;
            for example in examples.into_iter()
            {
                writeln!(f, "{example}")?;
            }
            writeln!(f)?;
        }

        write!(f, "{rules}", rules = style::info("One team on each line, followed by a ':', then the team's points for each round separated by whitespace."))
    }
}