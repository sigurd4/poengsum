use super::{Error, ErrorMsg};

#[derive(Debug)]
pub enum InsufficientData
{
    NoRoundsQueried,
    NoRoundsYet,
    RoundNotYet {
        round: usize,
        final_round: usize
    }
}

impl InsufficientData
{
    pub fn msg(&self) -> ErrorMsg
    {
        match self
        {
            InsufficientData::NoRoundsQueried => ErrorMsg {
                msg: "No results.".into(),
                error: None,
                line: None,
                hint: Some("The range of rounds provided is empty.".into()),
                docs: None
            },
            InsufficientData::NoRoundsYet => ErrorMsg {
                msg: "No results.".into(),
                error: None,
                line: None,
                hint: Some("None of the teams have gotten any points yet!".into()),
                docs: None
            },
            InsufficientData::RoundNotYet { round, final_round } => ErrorMsg {
                msg: format!("Round {round} hasn't happened yet.").into_boxed_str(),
                error: None,
                line: None,
                hint: Some(format!("There has only been {final_round} rounds so far!").into_boxed_str()),
                docs: None
            },
        }
    }
}

impl From<InsufficientData> for Error
{
    fn from(error: InsufficientData) -> Self
    {
        Self::InsufficientData {
            error
        }
    }
}