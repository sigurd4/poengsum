use super::{Error, Msg};

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
    pub fn msg(&self) -> Msg<'static>
    {
        match self
        {
            InsufficientData::NoRoundsQueried => Msg {
                msg: "No results.".into(),
                error: None,
                line: None,
                hint: Some("The range of rounds provided is empty.".into()),
                docs: None
            },
            InsufficientData::NoRoundsYet => Msg {
                msg: "No results.".into(),
                error: None,
                line: None,
                hint: Some("None of the teams have gotten any points yet!".into()),
                docs: None
            },
            InsufficientData::RoundNotYet { round, final_round } => Msg {
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