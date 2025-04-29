use std::{borrow::Cow, path::Path};

use crate::{error::Error, record::Records, round::Rounds, score::Scores};

pub struct Run
{
    rounds: Rounds,
    file: Cow<'static, Path>,
    rev: bool
}

impl Run
{
    pub fn new(rounds: Rounds, file: Cow<'static, Path>, rev: bool) -> Result<Self, Error>
    {
        Ok(Self {
            rounds,
            file,
            rev
        })
    }
    fn records(&self) -> Result<Records, Error>
    {
        Records::read(self).map_err(|e| e.at(self.file.clone()))
    }
    pub fn scores(self) -> Result<Scores, Error>
    {
        let records = self.records()?;
        Ok(Scores::new(records, self.rounds, self.rev)?)
    }
    pub fn file_path(&self) -> &Path
    {
        &self.file
    }
}