use core::{error::Error, fmt::Display, ops::Range};
use std::path::Path;

use colored::{Color, Colorize};

use crate::help::Docs;

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

        let severity_color = match severity
        {
            //Severity::Warning => Color::Yellow,
            Severity::Error => Color::Red,
        };

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
            format!("In {file} (line: {row}{at_col}){colon}").italic().color(severity_color)
        };
        write!(f, "{in_file}")?;

        if let Some(line) = *line
        {
            let [line_before, line, line_after] = match col
            {
                None => {
                    let arrow = "v".color(severity_color);
                    writeln!(f, "\n{arrow}")?;
                    ["", line, ""]
                },
                Some(Range { start, end }) => {
                    let (start, end) = (*start, *end);
                    let arrow = format!("{:#>start$}", "v").color(severity_color);
                    writeln!(f, "\n{arrow}")?;
                    let (line_before, line) = line.split_at(start);
                    let (line, line_after) = line.split_at(end);

                    [line_before, line, line_after]
                }
            };

            let lines = [
                line_before.bright_black(),
                line.bright_black().on_color(severity_color).underline(),
                line_after.bright_black()
            ];

            for line in lines
            {
                write!(f, "{line}")?;
            }
        }

        Ok(())
    }
}

pub struct ErrorMsg<'a>
{
    pub msg: Box<str>,
    pub error: Option<&'a dyn Error>,
    pub line: Option<OffendingLine<'a>>,
    pub hint: Option<Box<str>>,
    pub docs: Option<Docs<'a>>
}

impl Display for ErrorMsg<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self { msg, error, line, hint, docs } = self;

        let msg = format!("{msg}").italic().bright_black();
        write!(f, "{msg}")?;

        if let Some(error) = error
        {
            let error = format!("{error}").red();
            write!(f, "\n\n{error}")?;
        }

        if let Some(line) = line
        {
            write!(f, "\n\n{line}")?;
        }

        if let Some(docs) = docs
        {
            write!(f, "\n\n{docs}")?;
        }

        if let Some(hint) = hint
        {
            if line.is_some() || error.is_some() || docs.is_some()
            {
                write!(f, "\n")?;
            }
            let hint = format!("{hint}").italic().bright_black();
            write!(f, "\n{hint}")?;
        }

        Ok(())
    }
}