use core::{cmp::Ordering, fmt::Display};

use colored::Colorize;

use crate::{error::Error, record::Record};

#[derive(PartialEq, Clone)]
pub struct Score
{
    pub team: String,
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

impl Score
{
    pub fn present(mut scores: Vec<Score>)
    {
        Self::sort(&mut scores, |score| score, |score| score);
    
        for score in scores
        {
            println!("{}", score);
        }
    }

    fn sort<T>(scores: &mut Vec<T>, mut f: impl FnMut(&T) -> &Score, f_mut: impl FnMut(&mut T) -> &mut Score)
    {
        scores.sort_by(|a, b| f(a).partial_cmp(f(b)).unwrap_or(Ordering::Equal));

        let mut j = 0;
        let mut prev = None;

        for (i, score) in scores.iter_mut()
            .map(f_mut)
            .rev()
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

    fn scores_reduce_no_climb(records: Vec<Record>, mut reduce: impl FnMut(Vec<f64>) -> f64) -> Vec<Score>
    {
        let mut scores = records.into_iter()
            .enumerate()
            .map(move |(uid, record)| Score {
                team: record.team,
                points: reduce(record.points),
                climb: 0,
                plass: 0,
                uid
            }).collect();

        Self::sort(&mut scores, |score| score, |score| score);
        scores
    }

    fn scores_no_climb(records: Vec<Record>, runder: Option<Vec<usize>>) -> Vec<Score>
    {
        Self::scores_reduce_no_climb(
            records,
            |points| match &runder
            {
                Some(runder) => runder.iter()
                    .flat_map(|&runde| points.get(runde))
                    .copied()
                    .sum(),
                None => points.into_iter()
                    .sum()
            }
        )
    }

    fn siste_runde(records: &[Record]) -> Option<usize>
    {
        records.iter()
            .map(|record| record.points.len())
            .max()
            .and_then(|runde| runde.checked_sub(1))
    }

    fn check_runder(records: &[Record], runder: &Option<Vec<usize>>) -> Result<(), Error>
    {
        let siste_runde = Self::siste_runde(records).ok_or(Error::NoRoundsYet)?;

        if let Some(runder) = runder
        {
            for &runde in runder
            {
                if runde > siste_runde
                {
                    return Err(Error::RoundNotYet {
                        round: runde + 1,
                        last_round: siste_runde + 1
                    })
                }
            }
        }

        Ok(())
    }

    fn runder_som_var(records: &[Record], runder: &Option<Vec<usize>>) -> Option<Vec<usize>>
    {
        match runder
        {
            None => Self::siste_runde(records)
                .and_then(|siste_runde| siste_runde.checked_sub(1))
                .map(|sist_runde| (0..=sist_runde).collect()),
            Some(runder) => runder.len()
                .checked_sub(2)
                .map(|i| runder[..=i].to_vec())
        }
    }

    pub fn scores(records: Vec<Record>, runder: Option<Vec<usize>>) -> Result<Vec<Score>, Error>
    {
        Self::check_runder(&records, &runder)?;

        let prev_scores = if let Some(runder_som_var) = Self::runder_som_var(&records, &runder)
        {
            Some(Self::scores_no_climb(
                records.clone(),
                Some(runder_som_var)
            ))
        }
        else
        {
            None
        };

        let mut scores = Self::scores_no_climb(records, runder);

        if let Some(prev_scores) = prev_scores
        {
            for (i, score) in scores.iter_mut()
                .enumerate()
            {
                for (j, prev_score) in prev_scores.iter()
                    .enumerate()
                {
                    if prev_score.uid == score.uid
                    {
                        score.climb = i.checked_signed_diff(j).unwrap_or(0)
                    }
                }
            }
        }

        Ok(scores)
    }
}