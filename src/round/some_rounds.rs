use super::Round;

pub struct SomeRounds
{
    rounds: Vec<Round>
}

impl SomeRounds
{
    pub fn new(round: Round) -> Self
    {
        Self {
            rounds: vec![round]
        }
    }

    pub fn add_round(&mut self, round: Round)
    {
        self.rounds.push(round);
    }
}

impl IntoIterator for SomeRounds
{
    type IntoIter = <Vec<Round> as IntoIterator>::IntoIter;
    type Item = <Vec<Round> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter
    {
        self.rounds.into_iter()
    }
}