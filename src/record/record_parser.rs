use crate::error::InvalidSyntax;

use super::{Record, Records};

pub struct RecordParser
{
    row: usize,
    records: Vec<Record>
}

impl RecordParser
{
    pub fn new() -> Self
    {
        Self {
            row: 0,
            records: Vec::new()
        }
    }

    pub fn row(&self) -> usize
    {
        self.row
    }

    pub fn parse_line(&mut self, line: Box<str>) -> Result<(), InvalidSyntax>
    {
        self.row += 1;
        if line.trim().is_empty()
        {
            return Ok(())
        }
        let mut col = None;
        for i in 0..line.len()
        {
            if line.get(i..i + 1) == Some(":")
            {
                col = Some(i);
            }
        }
        let mut col = match col
        {
            Some(col) => col,
            None => return Err(InvalidSyntax::MissingColon {
                row: self.row,
                line
            })
        };

        let team = line.get(..col).map(str::trim).unwrap_or("");
        if team.is_empty()
        {
            return Err(InvalidSyntax::UnnamedTeam {
                row: self.row,
                line
            })
        }
        let team = Box::<str>::from(team);
        col += 1;
        let each_points = line.get(col..).map(str::trim).unwrap_or("");

        let mut points = Vec::new();

        for (i, span) in each_points.split_whitespace().enumerate()
        {
            col = unsafe {
                span.as_ptr().offset_from_unsigned(line.as_ptr())
            };
            points.push(match span.parse::<f64>()
            {
                Ok(points) => points,
                Err(error) => return Err(InvalidSyntax::CannotParsePoints {
                    row: self.row,
                    col: col..(col + span.len()),
                    line,
                    team,
                    round: i + 1,
                    error
                })
            });
        }

        self.records.push(Record {
            team,
            points
        });

        Ok(())
    }

    pub fn collect(self) -> Result<Records, InvalidSyntax>
    {
        Records::collect_from(self.records)
    }
}