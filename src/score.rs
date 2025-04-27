use core::{cmp::Ordering, fmt::Display};

use colored::Colorize;

use crate::{error::InsufficientData, record::Records, round::{BoundedRounds, Rounds}};

#[derive(PartialEq, Clone)]
pub struct Score
{
    pub team: Box<str>,
    pub points: f64,
    pub plass: usize,
    pub climb: isize,
    pub uid: usize
}
impl PartialOrd for Score
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.points.partial_cmp(&other.points)
        {
            None | Some(Ordering::Equal) => Some(
                self.climb.cmp(&other.climb)
                    .then(self.team.cmp(&other.team))
            ),
            cmp => cmp
        }
    }
}
impl Display for Score
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let climb = match self.climb.cmp(&0)
        {
            Ordering::Greater => format!(" ↑{}", self.climb.unsigned_abs()).green(),
            Ordering::Equal => "".into(),
            Ordering::Less => format!(" ↓{}", self.climb.unsigned_abs()).red()
        };

        write!(f, "{}. {}: {}{}", self.plass, self.team, self.points, climb)
    }
}

pub struct Scores
{
    scores: Vec<Score>
}

impl Scores
{
    pub fn present(mut self)
    {
        self.sort();
    
        for score in self.scores
        {
            println!("{score}");
        }
    }

    fn sort(&mut self)
    {
        self.scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

        let mut j = 0;
        let mut prev = None;

        for (i, score) in self.scores.iter_mut()
            .enumerate()
        {
            if Some(score.points) != prev
            {
                j = i + 1;
                prev = Some(score.points);
            }
            score.plass = j
        }
    }

    fn compared_to(&mut self, other: Scores)
    {
        for other in other.scores.into_iter()
        {
            for score in self.scores.iter_mut()
            {
                if other.uid == score.uid
                {
                    score.climb = crate::checked_signed_diff(other.plass, score.plass).unwrap_or(0)
                }
            }
        }
    }

    pub fn new(records: Records, rounds: Rounds) -> Result<Scores, InsufficientData>
    {
        fn scores_no_climb(records: Records, rounds: BoundedRounds) -> Scores
        {
            let mut scores = Scores {
                scores: records.into_iter()
                    .enumerate()
                    .map(move |(uid, record)| Score {
                        team: record.team,
                        points: rounds.iter()
                            .flat_map(|&round| round.index(&record.points))
                            .flatten()
                            .copied()
                            .sum(),
                        climb: 0,
                        plass: 0,
                        uid
                    }).collect::<Vec<_>>()
            };
            scores.sort();
            scores
        }

        let rounds = rounds.bound(records.final_round())?;
        
        let prev_scores = rounds.clone()
            .undo()
            .map(|prev_rounds| scores_no_climb(
                records.clone(),
                prev_rounds
            ));

        let mut scores = scores_no_climb(records, rounds);

        if let Some(prev_scores) = prev_scores
        {
            scores.compared_to(prev_scores)
        }

        Ok(scores)
    }
}