use core::{error::Error, fmt::Display, ops::Range};
use std::path::Path;

use crate::{help::Docs, style};

#[derive(Clone, Copy)]
pub enum Severity
{
    //Warning,
    Error
}

pub struct OffendingLine<'a>
{
    pub file: &'a Path,
    pub severity: Severity,
    pub line: Option<&'a str>,
    pub row: usize,
    pub col: Option<Range<usize>>
}

impl Display for OffendingLine<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self { file, severity, line, row, col } = self;

        let in_file = {
            let file = file.display();
            let at_col = match col.as_ref().map(|col| col.start)
            {
                None => "".into(),
                Some(col) => format!(":{col}")
            };
            let colon = if line.is_some()
            {
                ":"
            }
            else
            {
                ""
            };
            style::header(format!("In {file} (line: {row}{at_col}){colon}"))
        };
        write!(f, "{in_file}")?;

        if let Some(line) = *line
        {
            let row = format!("{row} | ");
            let [line_before, line, line_after] = match col
            {
                None => {
                    ["", line, ""]
                },
                Some(Range { start, end }) => {
                    let end = (*end).min(line.len());
                    let start = (*start).min(end);

                    let (line_before, line) = line.split_at_checked(start).unwrap_or((line, ""));
                    if line.is_empty() || start == end
                    {
                        [line_before, " ", line]
                    }
                    else
                    {
                        let (line, line_after) = line.split_at_checked(end - start).unwrap_or((line, ""));
                        [line_before, line, line_after]
                    }
                }
            };
            write!(f, "\n{arrow}\n{row}", arrow = style::syntax_arrow(line_before.chars().count() + row.chars().count(), *severity), row = style::row(row))?;

            let lines = [
                style::line(line_before),
                style::syntax_line(line, *severity),
                style::line(line_after)
            ];

            for line in lines
            {
                write!(f, "{line}")?;
            }
        }

        Ok(())
    }
}

pub struct Msg<'a>
{
    pub msg: Box<str>,
    pub error: Option<&'a dyn Error>,
    pub line: Option<OffendingLine<'a>>,
    pub hint: Option<Box<str>>,
    pub docs: Option<Docs<'a>>
}

impl Display for Msg<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self { msg, error, line, hint, docs } = self;

        let msg = style::info(format!("{msg}"));
        write!(f, "{msg}")?;

        if let Some(error) = error
        {
            let error = style::error(format!("Error: {error}."));
            write!(f, "\n\n{error}")?;
        }
        if let Some(line) = line
        {
            write!(f, "\n\n{line}")?;
        }

        if let Some(hint) = hint
        {
            if line.is_some() || error.is_some() //|| docs.is_some()
            {
                writeln!(f)?;
            }
            let hint = style::info(&**hint);
            write!(f, "\n{hint}")?;
        }

        if let Some(docs) = docs
        {
            write!(f, "\n\n{docs}")?;
        }

        Ok(())
    }
}