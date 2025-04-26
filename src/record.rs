use std::{fs::File, io::{BufRead, BufReader}};

use crate::{error::Error, POENGSUM_PATH};

#[derive(PartialEq, Clone)]
pub struct Record
{
    pub team: String,
    pub points: Vec<f64>
}

impl Record
{
    pub fn read() -> Result<Vec<Record>, Error>
    {
        let file = File::open(POENGSUM_PATH)
            .map_err(|error| Error::CannotOpenFile {
                error
            })?;
        
        let mut reader = BufReader::new(file);
        
        let mut records = Vec::new();
    
        let mut row = 0;
        let mut line = String::new();
        while {
            line.clear();
            reader.read_line(&mut line)
                .map_err(|error| Error::CannotReadFile {
                    row,
                    error
                })? != 0
        }
        {
            if line.trim().len() == 0
            {
                continue
            }
            let mut col = None;
            for i in 0..line.len()
            {
                if line.get(i..i + 1) == Some(":")
                {
                    col = Some(i);
                }
            }
            let col = match col
            {
                Some(col) => col,
                None => return Err(Error::MissingColon {
                    row,
                    line
                })
            };

            let team = line.get(..col).map(str::trim).unwrap_or("");
            if team.is_empty()
            {
                return Err(Error::UnnamedTeam {
                    row,
                    line
                })
            }
            let team = team.to_string();
            let each_points = line.get(col + 1..).map(str::trim).unwrap_or("");
    
            let mut points = Vec::new();
    
            for (i, span) in each_points.split_whitespace().enumerate()
            {
                points.push(match span.parse::<f64>()
                {
                    Ok(points) => points,
                    Err(error) => return Err(Error::CannotParsePoints {
                        row,
                        col,
                        span: span.to_string(),
                        line,
                        team,
                        round: i + 1,
                        error
                    })
                });
            }
    
            records.push(Record {
                team,
                points
            });
    
            row += 1;
        }

        if records.is_empty()
        {
            return Err(Error::NoTeams)
        }
    
        Ok(records)
    }
}