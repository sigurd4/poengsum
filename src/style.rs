use core::cmp::Ordering;
use std::borrow::Cow;

use ansi_term::{ANSIString, Color, Style};

use crate::{error::Severity, terminal};

const CLIMB_UP_COLOR: Color = Color::Green;
const CLIMB_DOWN_COLOR: Color = Color::Red;

const PLACE_COLOR: Color = Color::White;
const TEAM_COLOR: Color = Color::RGB(255/2, 255/2, 0);
const ROUND_COLOR: Color = Color::White;

const EXE_COLOR: Color = Color::Blue;
const ARG_COLOR: Color = Color::Cyan;

const INFO_COLOR: Color = Color::Fixed(8);
const ERROR_COLOR: Color = Color::Red;
//const WARNING_COLOR: Color = Color::Yellow;
const LINE_COLOR: Color = Color::Fixed(8);

pub fn place(place: usize) -> ANSIString<'static>
{
    PLACE_COLOR.paint(format!("{place}."))
}

pub fn team<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    TEAM_COLOR.paint(input)
}

pub fn points<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    ROUND_COLOR.paint(input)
}

pub fn header<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    Style::new().bold().paint(input)
}

pub fn climb(climb: isize) -> ANSIString<'static>
{
    let supports_unicode = supports_unicode::supports_unicode();
    let (arrow, color) = match climb.cmp(&0)
    {
        Ordering::Greater => (if supports_unicode {'↑'} else {'^'}, CLIMB_UP_COLOR),
        Ordering::Equal => return "".into(),
        Ordering::Less => (if supports_unicode {'↓'} else {'v'}, CLIMB_DOWN_COLOR)
    };
    color.paint(format!(" {}{}", arrow, climb.unsigned_abs()))
}

pub fn exe<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    EXE_COLOR.paint(input)
}
pub fn arg<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    ARG_COLOR.paint(input)
}
pub fn info<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    INFO_COLOR.italic().paint(input)
}
pub fn error<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    ERROR_COLOR.paint(input)
}

fn severity_color(severity: Severity) -> Color
{
    match severity
    {
        //Severity::Warning => WARNING_COLOR,
        Severity::Error => ERROR_COLOR
    }
}

pub fn syntax_arrow(offset: usize, severity: Severity) -> ANSIString<'static>
{
    severity_color(severity).paint(format!("{v:>offset$}", v = "v"))
}

pub fn line<'a, T>(input: T) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    LINE_COLOR.paint(input)
}
pub fn syntax_line<'a, T>(input: T, severity: Severity) -> ANSIString<'a>
where
    T: Into<Cow<'a, str>>
{
    let color = severity_color(severity);
    if terminal::supports_colored_underline() && let Some(prefix) = match color
    {
        Color::Fixed(n) => Some(format!("\x1b[58;5;{n}m")),
        Color::RGB(r, g, b) => Some(format!("\x1b[58;2;{r};{g};{b}m")),
        _ => None
    }
    {
        const SUFFIX: &str = "\x1b[58m";
        line(format!("{prefix}{input}{SUFFIX}", input = input.into()))
    }
    else
    {
        color.underline().paint(input)
    }
}