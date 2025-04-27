use core::{num::ParseFloatError, ops::Range};

use super::InvalidRead;

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
        line: Box<str>,
    },
    CannotParsePoints {
        row: usize,
        round: usize,
        col: Range<usize>,
        line: Box<str>,
        team: Box<str>,
        error: ParseFloatError
    },
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