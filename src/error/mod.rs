use core::fmt::Display;
use std::{borrow::Cow, io::ErrorKind, path::Path};

use crate::flag::Flag;

moddef::moddef!(
    flat(pub) mod {
        expected_arg,
        insufficient_data,
        invalid_arg,
        invalid_call,
        invalid_flag,
        invalid_io,
        invalid_read,
        invalid_syntax
    }
);

#[derive(thiserror::Error, Debug)]
pub enum Error
{
    InvalidRead {
        file: Cow<'static, Path>,
        error: InvalidRead
    },
    InvalidCall {
        no: usize,
        error: InvalidCall
    },
    InsufficientData {
        error: InsufficientData
    }
}

impl Error
{
    fn rules<'a>(file: &std::path::Display<'a>) -> String
    {
        format!(
            "Each line of \"{file}\" must be on the form `<team>: <points...>`\nOne team on each line, followed by a ':', then the team's points for each round separated by whitespace.\nExample: \"Quizzbuzz: 12 25 9\""
        )
    }
}

impl Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let nth = |no: usize| {
            let suffix = match no
            {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th"
            };
            format!("{no}{suffix}")
        };
        match self
        {
            Self::InvalidRead { file, error } => {
                let file = file.display();
                match error
                {
                    InvalidRead::InvalidIO { io_error, error } => {
                        let hint = |placeholder: &str| match io_error.kind()
                        {
                            ErrorKind::NotFound => {
                                let rules = Self::rules(&file);
                                format!("\nIf it doesn't exist, create it!\n\n{rules}")
                            },
                            ErrorKind::PermissionDenied => "\nYou don't have permission to view this file.".to_string(),
                            ErrorKind::IsADirectory => format!("\nThere's, for some reason, a directory with the same name as {file}.\nThis program needs {file} to be a file, not a directory."),
                            ErrorKind::Other => placeholder.to_string(),
                            _ => "".to_string(),
                        };
                        match error
                        {
                            InvalidIO::Open => {
                                let hint = hint("Oops!");
                                write!(f,
                                    "Failed to open file \"{file}\".\n{io_error}.{hint}"
                                )
                            },
                            InvalidIO::Read { row } => {
                                let hint = hint("Maybe the file is busy?");
                                write!(f,
                                    "Failed to read line {row} of file \"{file}\".\n{io_error}.{hint}"
                                )
                            },
                        }
                    },
                    InvalidRead::InvalidSyntax { error } => {
                        let rules = Self::rules(&file);
                        match error
                        {
                            InvalidSyntax::MissingColon { row, line } => write!(f,
                                "Seperator ':' missing at line {row}.\n\n{row}| {line}\n\nUse a ':' to separate the team name and their points for each round.\n\n{rules}"
                            ),
                            InvalidSyntax::UnnamedTeam { row, line } => write!(f,
                                "Team name at line {row} is empty. Give that team a name!\n\n{row}| {line}\n\n{rules}"
                            ),
                            InvalidSyntax::NoTeams => write!(f,
                                "The file \"{file}\" is empty!\nYou need to add some teams to your \"{file}\"-file.\n\n{rules}"
                            ),
                            InvalidSyntax::CannotParsePoints { row, round, col, line, team, error } => {
                                let span = &line[col.clone()];
                                let col = col.start;
                                write!(f,
                                    "Unable to parse points for team \"{team}\" for round {round} at line {row}, collumn {col} \"{span}\": {error}\n\n{row}| {line}\n\n\"{span}\" must be a valid number."
                                )
                            },
                        }
                    },
                }
            },
            Self::InvalidCall { no, error } => {
                let nth = nth(*no);
                match error
                {
                    InvalidCall::InvalidArg { arg, error } => {
                        let arg = arg.as_ref()
                            .map(|arg| format!(" \"{arg}\""))
                            .unwrap_or_else(String::new);
                        match error
                        {
                            InvalidArg::UseTwoDots => write!(f,
                                "The {nth} argument{arg} is invalid.\nDid you mean to write '..' instead of '.'?"
                            ),
                            InvalidArg::CannotParseInteger { error } => write!(f,
                                "Unable to parse the {nth} commandline argument{arg}: {error}.\nThis must be a valid number (a nonzero integer)."
                            ),
                            InvalidArg::NotInOrder { start, end } => write!(f,
                                "The {nth} argument{arg} is invalid.\nRanges cannot be in reverse.\n{start} is larger than {end}, but this is not supported."
                            ),
                            InvalidArg::RoundZero => write!(f,
                                "The {nth} argument{arg} is invalid.\n0 is not a valid round! Rounds start at 1, not 0."
                            ),
                            InvalidArg::NonexistentFlag { flag } => {
                                let options = Flag::OPTIONS.into_iter()
                                    .map(|flag| format!("\n\t--{flag}"))
                                    .collect::<String>();
                                write!(f,
                                    "The {nth} argument{arg} is invalid.\nThere is no available option with the name \"{flag}\".\nAvailable options are:{options}"
                                )
                            },
                            InvalidArg::InvalidFlag { error } => match error
                            {
                                InvalidFlag::FileAlreadySpecified => {
                                    let default_file = crate::default_file_path().display();
                                    write!(f,
                                        "The {nth} argument{arg} is invalid.\nYou've already specified a filename.\nBy default, the file that the score is read from is \"{default_file}\", but you can use a different file by setting the \"--file\" flag, followed by a path.\nYou're not allowed to set multiple files."
                                    )
                                }
                            }
                        }
                    },
                    InvalidCall::ExpectedArg { error } => match error
                    {
                        ExpectedArg::Filename => write!(f,
                            "Expected a filename as the {nth} argument."
                        )
                    }
                }
            },
            Self::InsufficientData { error } => match error
            {
                InsufficientData::NoRoundsQueried => write!(f,
                    "No results. The range of rounds provided is empty."
                ),
                InsufficientData::NoRoundsYet => write!(f,
                    "No results. None of the teams have gotten any points yet!"
                ),
                InsufficientData::RoundNotYet { round, final_round: last_round } => write!(f,
                    "Round {round} hasn't happened yet.\nThere has only been {last_round} rounds so far!"
                ),
            }
        }
    }
}