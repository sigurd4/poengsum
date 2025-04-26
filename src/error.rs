use core::{fmt::Display, str::FromStr};
use std::io::ErrorKind;

use crate::POENGSUM_PATH;

#[derive(thiserror::Error, Debug)]
pub enum Error
{
    CannotOpenFile {
        error: std::io::Error
    },
    CannotReadFile {
        row: usize,
        error: std::io::Error
    },
    MissingColon {
        row: usize,
        line: String
    },
    UnnamedTeam {
        row: usize,
        line: String
    },
    CannotParsePoints {
        row: usize,
        col: usize,
        line: String,
        span: String,
        team: String,
        round: usize,
        error: <f64 as FromStr>::Err
    },
    CannotParseRound {
        no: usize,
        arg: String,
        error: <usize as FromStr>::Err
    },
    RoundZero {
        no: usize
    },
    NoTeams,
    NoRoundsYet,
    RoundNotYet {
        round: usize,
        last_round: usize
    }
}
impl Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let rules = format!(
            "Each line of \"{POENGSUM_PATH}\" must be on the form `<team>: <points...>`\nOne team on each line, followed by a ':', then the team's points for each round separated by whitespace.\nExample: \"Quizzbuzz: 12 25 9\""
        );
        match self
        {
            Self::CannotOpenFile { error } => {
                let hint = match error.kind()
                {
                    ErrorKind::NotFound => format!("\nIf it doesn't exist, create it!\n\n{rules}"),
                    ErrorKind::PermissionDenied => "\nYou don't have permission to view this file.".to_string(),
                    ErrorKind::IsADirectory => format!("\nThere's, for some reason, a directory with the same name as {POENGSUM_PATH}.\nThis program needs {POENGSUM_PATH} to be a file, not a directory."),
                    ErrorKind::Other => "\nOops!".to_string(),
                    _ => "".to_string(),
                };
                write!(f,
                    "Cannot open file \"{POENGSUM_PATH}\".\n{error}.{hint}"
                )
            },
            Self::CannotReadFile { row, error } => {
                let hint = match error.kind()
                {
                    ErrorKind::NotFound => "\nIf it doesn't exist, create it!",
                    ErrorKind::PermissionDenied => "\nYou don't have permission to view this file.",
                    ErrorKind::IsADirectory => "\nThat's not a file. That's a directory!",
                    ErrorKind::Other => "\nMaybe the file is just busy?",
                    _ => "",
                };
                write!(f,
                    "Cannot read line {row} in file \"{POENGSUM_PATH}\".\n{error}.{hint}"
                )
            },
            Self::MissingColon { row, line } => write!(f,
                "Seperator ':' missing at line {row}.\n\n{row}| {line}\n\nUse a ':' to separate the team name and their points for each round.\n\n{rules}"
            ),
            Self::UnnamedTeam { row, line } => write!(f,
                "Team name at line {row} is empty. Give that team a name!\n\n{row}| {line}\n\n{rules}"
            ),
            Self::CannotParsePoints { row, col, line, span, team, round, error } => write!(f,
                "Unable to parse points for team \"{team}\" for round {round} at line {row}, collumn {col} \"{span}\": {error}\n\n{row}| {line}\n\n\"{span}\" must be a valid number."
            ),
            Self::CannotParseRound { no, arg, error } => write!(f,
                "Unable to parse commandline argument #{no} of \"{arg}\": {error}\nThis must be a valid number (a nonzero integer)."
            ),
            Self::RoundZero { no } => write!(f,
                "Invalid commandline argument #{no} of \"0\".\n0 is not a valid round! Rounds start at 1, not 0."
            ),
            Self::NoTeams => write!(f,
                "The file \"{POENGSUM_PATH}\" is empty!\nYou need to add some teams to your \"{POENGSUM_PATH}\"-file.\n\n{rules}"
            ),
            Self::NoRoundsYet => write!(f,
                "No results. None of the teams have gotten any points yet!"
            ),
            Self::RoundNotYet { round, last_round } => write!(f,
                "Round {round} hasn't happened yet.\nThere has only been {last_round} so far!"
            )
        }
    }
}