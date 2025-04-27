use crate::error::InsufficientData;

use super::{BoundedRound, BoundedRounds, Round, SomeRounds};

pub enum Rounds
{
    All,
    Some(SomeRounds)
}

impl Rounds
{
    pub fn add_round(&mut self, round: Round)
    {
        if let Rounds::Some(rounds) = self
        {
            rounds.add_round(round)
        }
        else
        {
            *self = Rounds::Some(SomeRounds::new(round))
        }
    }

    pub fn bound(self, final_round: Option<usize>) -> Result<BoundedRounds, InsufficientData>
    {
        match self
        {
            Self::All => BoundedRounds::new([
                BoundedRound::new(0, Some(final_round.ok_or(InsufficientData::NoRoundsYet)?))
            ]),
            Self::Some(rounds) => {
                let rounds = rounds.into_iter()
                    .map(|round| round.bound(final_round));
                BoundedRounds::try_new(rounds)
            }
        }
    }
}