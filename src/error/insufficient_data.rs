use super::Error;

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

impl From<InsufficientData> for Error
{
    fn from(error: InsufficientData) -> Self
    {
        Self::InsufficientData {
            error
        }
    }
}