use std::{fs::File, io::{BufRead, BufReader}};

use crate::{error::{InvalidIO, InvalidRead, InvalidSyntax}, run::Run};

use super::{Record, RecordParser};

#[derive(Clone)]
pub struct Records
{
    records: Vec<Record>
}

impl Records
{
    pub fn final_round(&self) -> Option<usize>
    {
        self.records.iter()
            .map(|record| record.points.len())
            .max()
            .and_then(|runde| runde.checked_sub(1))
    }

    pub fn collect_from(records: impl IntoIterator<Item = Record>) -> Result<Self, InvalidSyntax>
    {
        let records = records.into_iter()
            .collect::<Vec<_>>();

        if records.is_empty()
        {
            return Err(InvalidSyntax::NoTeams)
        }

        Ok(Self {
            records
        })
    }

    pub fn read(run: &Run) -> Result<Self, InvalidRead>
    {
        let file_path = run.file_path();
        let file = File::open(file_path)
            .map_err(|io_error| InvalidRead::InvalidIO {
                io_error,
                error: InvalidIO::Open
            })?;
        
        let reader = BufReader::new(file);

        let mut parser = RecordParser::new();
        for line in reader.lines()
        {
            let line = line.map_err(|io_error| InvalidRead::InvalidIO {
                    io_error,
                    error: InvalidIO::Read {
                        row: parser.row()
                    }
                })?.into_boxed_str();
            parser.parse_line(line)?
        }

        Ok(parser.collect()?)
    }
}

impl IntoIterator for Records
{
    type IntoIter = <Vec<Record> as IntoIterator>::IntoIter;
    type Item = <Vec<Record> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter
    {
        self.records.into_iter()
    }
}