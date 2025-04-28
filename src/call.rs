use std::{borrow::Cow, path::{Path, PathBuf}};

use crate::{error::{ArgError, Error, ExpectedArg, InvalidArg, InvalidFlag}, flag::{Flag, FlagKind, FlagOption}, help::Help, round::{Round, Rounds}, run::Run};

enum FlagBuilder
{
    File
}

impl FlagBuilder
{
    fn new(flag: FlagOption<&str>, parser: &RunBuilder, exe: &'static str) -> Result<Result<Flag, Self>, InvalidArg>
    {
        let flag = FlagKind::try_from(flag)?;
        match flag
        {
            FlagKind::Help => Ok(Ok(Flag::Help(Help::new(exe)))),
            FlagKind::File => if let Cow::Owned(_) = &parser.file //TODO: use .is_owned() when stabilized
            {
                Err(InvalidArg::InvalidFlag {
                    error: InvalidFlag::FileAlreadySpecified
                })
            }
            else
            {
                Ok(Err(Self::File))
            }
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

    fn replace_and_collect(self, replace: &mut Option<FlagBuilder>) -> Result<Option<Flag>, ExpectedArg>
    {
        if let Some(builder) = replace.replace(self) && let Some(flag) = builder.collect()?
        {
            Ok(Some(flag))
        }
        else
        {
            Ok(None)
        }
    }
}

pub struct RunBuilder
{
    exe: Option<&'static str>,
    flag_builder: Option<FlagBuilder>,
    rounds: Rounds,
    file: Cow<'static, Path>,
    help: Option<Help>,
    no: usize
}

impl RunBuilder
{
    pub fn new() -> Self
    {
        Self {
            exe: None,
            flag_builder: None,
            rounds: Rounds::All,
            file: Cow::Borrowed(crate::default_file_path()),
            help: None,
            no: 0
        }
    }

    pub fn from_args(args: impl IntoIterator<Item = String>) -> Result<Self, Error>
    {
        let mut parser = Self::new();
        parser.parse(args)?;
        Ok(parser)
    }

    fn next(&mut self, iter: &mut impl Iterator<Item = String>) -> Option<(&'static str, String)>
    {
        let exe = match self.exe
        {
            Some(exe) => exe,
            None => {
                self.exe = iter.next().map(|next| next.leak() as &str);
                match self.exe
                {
                    Some(exe) => exe,
                    None => return None
                }
            }
        };
        let next = iter.next();
        next.and_then(|next| {
            self.no += 1;

            Some((exe, next))
        })
    }

    fn add_flag(&mut self, flag: Flag) -> Result<(), InvalidFlag>
    {
        let add = || match flag
        {
            Flag::Help(help) => {
                match &mut self.help
                {
                    Some(h) => h.and(help),
                    None => self.help = Some(help)
                }
                Ok(())
            }
            Flag::File(file) => if let Cow::Borrowed(_) = &self.file
            {
                self.file = Cow::Owned(file);
                Ok(())
            }
            else
            {
                Err(InvalidFlag::FileAlreadySpecified)
            },
        };
        Help::catch(add(), (), self.help.as_mut())
    }

    fn add_flag_option(&mut self, flag: FlagOption<&str>, exe: &'static str) -> Result<(), ArgError>
    {
        if let Some(result) = Help::catch(FlagBuilder::new(flag, self, exe), None, self.help.as_mut())? && let Some(flag) = match result
        {
            Ok(flag) => Some(flag),
            Err(flag_builder) => Help::catch(flag_builder.replace_and_collect(&mut self.flag_builder), None, self.help.as_mut())?,
        }
        {
            self.add_flag(flag)?
        }

        Ok(())
    }

    fn parse_round(arg: &str) -> Result<Round, InvalidArg>
    {
        let offs = |b: usize| b.checked_sub(1)
            .ok_or(InvalidArg::RoundZero);

        let parse_int_notrim = |s: &str| s.parse::<usize>()
            .map_err(InvalidArg::from);

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
            let round = parse_int(arg)?;
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
        while let Some((exe, arg)) = self.next(&mut iter)
        {
            macro_rules! try_of {
                ($e:expr) => {
                    match $e
                    {
                        Ok(b) => b,
                        Err(e) => return Err(e.at(exe, self.no, Some(arg.into_boxed_str())))
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

            fn parse_trimmed(parser: &mut RunBuilder, exe: &'static str, arg: &str) -> Result<(), ArgError>
            {
                if let Some(flags) = arg.trim().strip_prefix("-")
                {
                    match flags.strip_prefix("-")
                    {
                        Some(flag) => parser.add_flag_option(FlagOption::Long(flag), exe)?,
                        None => for flag in flags.chars()
                        {
                            parser.add_flag_option(FlagOption::Short(flag), exe)?;
                        }
                    }

                    return Ok(())
                }
                else if parser.help.is_some()
                {
                    return Err(InvalidArg::IntegerAfterHelp.into())
                }
                else
                {
                    parser.rounds.add_round(RunBuilder::parse_round(arg)?);
                }

                Ok(())
            }

            try_of!(parse_trimmed(self, exe, arg.trim()))
        }

        Ok(())
    }

    pub fn collect(mut self) -> Result<Run, Error>
    {
        let exe = self.exe.ok_or(Error::NoExecutable)?;

        if let Some(flag_parser) = self.flag_builder.take() && let Some(flag) = Help::catch(flag_parser.collect(), None, self.help.as_mut()).map_err(|e| e.at(exe, self.no))?
        {
            self.add_flag(flag).map_err(|e| e.at(exe, self.no, None))?;
        }

        let Self { exe: _, flag_builder, rounds, file, help, no } = self;
        let _ = (flag_builder, no);

        if let Some(help) = help
        {
            return Err(Error::ShowHelp {
                help
            })
        }

        Run::new(rounds, file)
    }
}