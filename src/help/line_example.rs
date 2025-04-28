use core::fmt::Display;

use colored::Colorize;

use super::ALIGN;

pub struct LineExample
{
    pub team: Box<str>,
    pub points: Box<str>,
    pub effect: Option<Box<str>>
}

impl Display for LineExample
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let Self {team, points, effect} = self;
        let mut align = ALIGN;

        let team = format!("{team}:").blue();
        let points = points.white();

        align = align.saturating_sub(team.len() + points.len() + 2);
        write!(f, "{team} {points}")?;

        if let Some(effect) = effect.as_ref()
        {
            let effect = effect.italic().bright_black();
            write!(f, "{empty:>align$} {effect}", empty = "")?;
        }

        Ok(())
    }
}