use crate::error::InsufficientData;

use super::BoundedRound;

#[derive(Clone, Copy)]
pub enum Round
{
    One(usize),
    Range {
        start: Option<usize>,
        end: Option<usize>
    }
}

impl Round
{
    pub fn bound(self, final_round: Option<usize>) -> Result<BoundedRound, InsufficientData>
    {
        let final_round = final_round.ok_or(InsufficientData::NoRoundsYet)?;

        let check = |round: usize| if round <= final_round
        {
            Ok(round)
        }
        else
        {
            Err(InsufficientData::RoundNotYet { round, final_round })
        };

        match self
        {
            Round::One(round) => Ok(BoundedRound::One(check(round)?)),
            Round::Range { start, end } => {
                let check_or = |round: Option<usize>, or: usize| match round
                {
                    Some(round) => check(round),
                    None => Ok(or)
                };

                let start = check_or(start, 0)?;
                let end = check_or(end, final_round)?;

                Ok(BoundedRound::new(start, Some(end)))
            },
        }
    }
}