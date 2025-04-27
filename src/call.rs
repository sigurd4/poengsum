use std::{borrow::Cow, path::{Path, PathBuf}};

use crate::{error::{Error, ExpectedArg, InvalidArg, InvalidFlag}, flag::Flag, round::{Round, Rounds}, run::Run};

enum FlagBuilder
{
    File
}

impl FlagBuilder
{
    fn new(flag: &str, parser: &RunBuilder) -> Result<Self, InvalidArg>
    {
        match flag
        {
            "file" => if let Cow::Owned(_) = &parser.file //.is_owned()
            {
                Err(InvalidArg::InvalidFlag {
                    error: InvalidFlag::FileAlreadySpecified
                })
            }
            else
            {
                Ok(Self::File)
            },
            _ => Err(InvalidArg::NonexistentFlag {
                flag: flag.to_string()
            })
        }
    }

    fn parse(self, _no: usize, arg: String) -> Result<Result<Flag, Self>, Error>
    {
        match self
        {
            FlagBuilder::File => Ok(Ok(Flag::File(PathBuf::from(arg))))
        }
    }

    fn collect(self) -> Result<Option<Flag>, ExpectedArg>
    {
        match self
        {
            FlagBuilder::File => Err(ExpectedArg::Filename)
        }
    }
}

pub struct RunBuilder
{
    flag_builder: Option<FlagBuilder>,
    rounds: Rounds,
    file: Cow<'static, Path>,
    no: usize
}

impl RunBuilder
{
    pub fn new() -> Self
    {
        Self {
            flag_builder: None,
            rounds: Rounds::All,
            file: Cow::Borrowed(crate::default_file_path()),
            no: 0
        }
    }

    pub fn from_args(args: impl IntoIterator<Item = String>) -> Result<Self, Error>
    {
        let mut args = args.into_iter();
        let _ = args.next();
        
        let mut parser = Self::new();
        parser.parse(args)?;
        Ok(parser)
    }

    fn next(&mut self, iter: &mut impl Iterator<Item = String>) -> Option<String>
    {
        let next = iter.next();
        if next.is_some()
        {
            self.no += 1;
        }
        next
    }

    fn add_flag(&mut self, flag: Flag) -> Result<(), InvalidFlag>
    {
        match flag
        {
            Flag::File(file) => if let Cow::Borrowed(_) = &self.file
            {
                self.file = Cow::Owned(file);
                Ok(())
            }
            else
            {
                Err(InvalidFlag::FileAlreadySpecified)
            },
        }
    }

    fn parse_round(arg: &str) -> Result<Round, InvalidArg>
    {
        let offs = |b: usize| b.checked_sub(1)
            .map(|b| b)
            .ok_or(InvalidArg::RoundZero);

        let parse_int_notrim = |s: &str| s.parse::<usize>()
            .map_err(|error| InvalidArg::from(error));

        let parse_int = |s: &str| offs(parse_int_notrim(s.trim())?);

        let mut iter = if arg.contains("..")
        {
            arg.split_terminator("..")
        }
        else if arg.contains(".")
        {
            return Err(InvalidArg::UseTwoDots)
        }
        else
        {
            let round = parse_int(&arg)?;
            return Ok(Round::One(round))
        };

        let parse_bound = |mut s: &str| {
            s = s.trim();
            match s.is_empty()
            {
                true => Ok(None),
                false => offs(parse_int_notrim(s)?).map(Some)
            }
        };

        let start = match iter.next()
        {
            Some(s) => parse_bound(s)?,
            None => None
        };
        let mut end = match iter.next()
        {
            Some(s) => parse_bound(s)?,
            None => None
        };

        let check = |start: Option<usize>, end: Option<usize>| {
            if let (Some(start), Some(end)) = (start, end)
            {
                if start > end
                {
                    return Err(InvalidArg::NotInOrder {
                        start: start + 1,
                        end: end + 1
                    })
                }
            }

            Ok(())
        };

        check(start, end)?;
        for s in iter
        {
            let start = end;
            end = parse_bound(s)?;
            check(start, end)?;
        }

        Ok(Round::Range {
            start,
            end
        })
    }

    pub fn parse(&mut self, iter: impl IntoIterator<Item = String>) -> Result<(), Error>
    {
        let mut iter = iter.into_iter();
        while let Some(arg) = self.next(&mut iter)
        {
            macro_rules! try_of {
                ($e:expr) => {
                    match $e
                    {
                        Ok(b) => b,
                        Err(e) => return Err(e.at(self.no, Some(arg)))
                    }
                };
            }

            if let Some(flag_parser) = self.flag_builder.take()
            {
                match flag_parser.parse(self.no, arg.clone() /* :( */)?
                {
                    Ok(flag) => try_of!(self.add_flag(flag)),
                    Err(flag_parser) => self.flag_builder = Some(flag_parser),
                };
                continue
            }

            fn parse_trimmed(parser: &mut RunBuilder, arg: &str) -> Result<(), InvalidArg>
            {
                if let Some(flag) = arg.trim().strip_prefix("--")
                {
                    parser.flag_builder = Some(FlagBuilder::new(flag, &parser)?);
                    return Ok(())
                }
                else
                {
                    parser.rounds.add_round(RunBuilder::parse_round(&arg)?);
                }

                Ok(())
            }

            try_of!(parse_trimmed(self, arg.trim()))
        }

        Ok(())
    }

    pub fn collect(mut self) -> Result<Run, Error>
    {
        if let Some(flag_parser) = self.flag_builder.take() && let Some(flag) = flag_parser.collect().map_err(|e| e.at(self.no))?
        {
            self.add_flag(flag).map_err(|e| e.at(self.no, None))?;
        }

        let Self { flag_builder: _, rounds, file, no: _ } = self;

        Run::new(rounds, file)
    }
}