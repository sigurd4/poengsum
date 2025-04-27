use core::str::FromStr;

use super::InvalidRead;

#[derive(Debug)]
pub enum InvalidSyntax
{
    NoTeams,
    MissingColon {
        row: usize,
        line: String,
    },
    UnnamedTeam {
        row: usize,
        line: String,
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