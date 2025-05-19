use core::{cmp::Ordering, fmt::Display};

use crate::{error::InsufficientData, record::Records, round::{BoundedRounds, Rounds}, style};

#[derive(PartialEq, Clone)]
pub struct Score
{
    pub team: Box<str>,
    pub points: f64,
    pub place: usize,
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
        let Self { team, points, place, climb, uid: _ } = self;
        let points = if *points == -0.0
        {
            0.0
        }
        else
        {
            *points
        };
        write!(f, "{place} {team} {points}{climb}",
            place = style::place(*place),
            team = style::team(format!("{team}:", team = &**team).as_str()),
            points = style::points(format!("{points}").as_str()),
            climb = style::climb(*climb)
        )
    }
}

pub struct Scores
{
    scores: Vec<Score>,
    rev: bool
}

impl Scores
{
    pub fn present(mut self)
    {
        self.sort();
    
        if self.rev
        {
            for score in self.scores.into_iter().rev()
            {
                println!("{score}");
            }
        }
        else
        {
            for score in self.scores
            {
                println!("{score}");
            }
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
            score.place = j
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
                    score.climb = crate::checked_signed_diff(other.place, score.place).unwrap_or(0)
                }
            }
        }
    }

    pub fn new(records: Records, rounds: Rounds, rev: bool) -> Result<Scores, InsufficientData>
    {
        fn scores_no_climb(records: Records, rounds: BoundedRounds, rev: bool) -> Scores
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
                        place: 0,
                        uid
                    }).collect::<Vec<_>>(),
                rev
            };
            scores.sort();
            scores
        }

        let rounds = rounds.bound(records.final_round())?;
        
        let prev_scores = rounds.clone()
            .undo()
            .map(|prev_rounds| scores_no_climb(
                records.clone(),
                prev_rounds,
                rev
            ));

        let mut scores = scores_no_climb(records, rounds, rev);

        if let Some(prev_scores) = prev_scores
        {
            scores.compared_to(prev_scores)
        }

        Ok(scores)
    }
}