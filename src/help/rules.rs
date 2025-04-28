use core::fmt::Display;
use std::path::Path;

use colored::Colorize;

use super::LineExample;

pub struct Rules<'a>
{
    pub file: &'a Path
}

impl<'a> Display for Rules<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let file = self.file.display();
        let on_the_form = format!("Each line of \"{file}\" must be on the form:").bright_black().italic();
        let form = LineExample {
            team: "$team".into(),
            points: "$($points)*".into(),
            effect: Some("One team on each line, followed by a ':', then the team's points for each round separated by whitespace.".into())
        };
        let example = LineExample {
            team: "Quizzbuzz".into(),
            points: "12 25 9".into(),
            effect: Some("A team that got 12 points in the first round, then 25, and then only 9 in the final.".into())
        };
        write!(f, 
            "{on_the_form}\n\t{form}\n\nExample:\n\t{example}"
        )
    }
}