use core::fmt::Display;

use crate::style;

use super::ALIGN;

pub struct LineExample
{
    pub team: Box<str>,
    pub points: Box<str>,
    pub effect: Option<Box<str>>,
    pub row: usize
}

impl Display for LineExample
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self {team, points, effect, row} = self;
        let mut align = ALIGN;

        let team = style::team_line(format!("{team}:"));
        let points = style::points_line(&**points);

        align = align.saturating_sub(team.len() + points.len() + 2);
        write!(f, "{row} | {team} {points}")?;

        if let Some(effect) = effect.as_ref()
        {
            let effect = style::info(&**effect);
            write!(f, "{empty:>align$} {effect}", empty = "")?;
        }

        Ok(())
    }
}