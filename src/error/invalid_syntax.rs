use core::{num::ParseFloatError, ops::Range};
use std::path::Path;

use crate::help::{Docs, Rules};

use super::{Msg, InvalidRead, Severity, OffendingLine};

#[derive(Debug)]
pub enum InvalidSyntax
{
    NoTeams,
    MissingColon {
        row: usize,
        line: Box<str>,
    },
    UnnamedTeam {
        row: usize,
        col: Range<usize>,
        line: Box<str>,
    },
    CannotParsePoints {
        row: usize,
        col: Range<usize>,
        round: usize,
        line: Box<str>,
        team: Box<str>,
        error: ParseFloatError
    },
}

impl InvalidSyntax
{
    pub fn msg<'a>(&'a self, file: &'a Path) -> Msg<'a>
    {
        let file_display = file.display();
        match self
        {
            InvalidSyntax::NoTeams => Msg {
                msg: format!("The file \"{file_display}\" is empty!").into_boxed_str(),
                error: None,
                line: Some(OffendingLine {
                    file,
                    severity: Severity::Error,
                    line: Some(""),
                    row: 0,
                    col: None
                }),
                hint: Some(format!("You need to add some teams to your \"{file_display}\"-file.").into_boxed_str()),
                docs: Some(Docs::Rules(Rules {
                    file
                }))
            },
            InvalidSyntax::MissingColon { row, line } => Msg {
                msg: format!("Seperator ':' missing at line {row}.").into_boxed_str(),
                error: None,
                line: Some(OffendingLine {
                    file,
                    severity: Severity::Error,
                    line: Some(&**line),
                    row: *row,
                    col: None
                }),
                hint: Some("Use a ':' to separate the team name and their points for each round.".into()),
                docs: Some(Docs::Rules(Rules {
                    file
                }))
            },
            InvalidSyntax::UnnamedTeam { row, col, line } => Msg {
                msg: format!("Team name at line {row} is empty.").into_boxed_str(),
                error: None,
                line: Some(OffendingLine {
                    file,
                    severity: Severity::Error,
                    line: Some(&**line),
                    row: *row,
                    col: Some(col.clone())
                }),
                hint: Some("Give that team a name!".into()),
                docs: Some(Docs::Rules(Rules {
                    file
                }))
            },
            InvalidSyntax::CannotParsePoints { row, col, round, line, team, error } => {
                let span = &line[col.clone()];
                let col_start = col.start;
                Msg {
                    msg: format!("Unable to parse points for team \"{team}\" for round {round} at line {row}, collumn {col_start} \"{span}\": {error}\n\n{row}| {line}\n\n").into_boxed_str(),
                    error: Some(error),
                    line: Some(OffendingLine {
                        file,
                        severity: Severity::Error,
                        line: Some(&**line),
                        row: *row,
                        col: Some(col.clone())
                    }),
                    hint: Some(format!("\"{span}\" must be a valid number.").into_boxed_str()),
                    docs: Some(Docs::Rules(Rules {
                        file
                    }))
                }
            },
        }
    }
}

impl From<InvalidSyntax> for InvalidRead
{
    fn from(error: InvalidSyntax) -> Self
    {
        InvalidRead::InvalidSyntax {
            error
        }    
    }
}