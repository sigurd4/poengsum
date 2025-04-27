use crate::error::InsufficientData;

use super::BoundedRound;

#[derive(Clone)]
pub struct BoundedRounds
{
    rounds: Vec<BoundedRound>
}

impl BoundedRounds
{
    pub fn new(rounds: impl IntoIterator<Item = BoundedRound>) -> Result<Self, InsufficientData>
    {
        let rounds = rounds.into_iter().collect::<Vec<_>>();
        if rounds.is_empty()
        {
            return Err(InsufficientData::NoRoundsQueried)
        }
        Ok(Self {
            rounds
        })
    }
    pub fn try_new<E>(rounds: impl IntoIterator<Item = Result<BoundedRound, E>>) -> Result<Self, InsufficientData>
    where
        InsufficientData: From<E>
    {
        let rounds = crate::try_collect::<Vec<_>, _, _>(&mut rounds.into_iter())?;
        if rounds.is_empty()
        {
            return Err(InsufficientData::NoRoundsQueried)
        }
        Ok(Self {
            rounds
        })
    }

    pub fn undo(mut self) -> Option<Self>
    {
        if let Some(last) = self.rounds.pop()
        {
            match last.undo()
            {
                Some(last) => self.rounds.push(last),
                None => if self.rounds.is_empty()
                {
                    return None
                }
            }
        }

        Some(self)
    }

    pub fn iter(&self) -> core::slice::Iter<'_, BoundedRound>
    {
        self.rounds.iter()
    }
}