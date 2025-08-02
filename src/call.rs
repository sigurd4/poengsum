use std::{borrow::Cow, path::{Path, PathBuf}};

use crate::{error::{ArgError, Error, ExpectedArg, InvalidArg, InvalidFlag}, flag::{Flag, FlagKind, FlagOption}, help::Help, round::{Round, Rounds}, run::Run};

enum FlagCall
{
    File
}

impl FlagCall
{
    fn new(flag: FlagOption<&str>, call: &Call, exe: &'static str) -> Result<Result<Flag, Self>, InvalidArg>
    {
        let flag = FlagKind::try_from(flag)?;
        match flag
        {
            FlagKind::Help => if let Rounds::All = call.rounds
            {
                Ok(Ok(Flag::Help(Help::new(exe))))
            }
            else
            {
                Err(InvalidFlag::HelpAfterInteger.into())
            },
            FlagKind::File => if let Cow::Owned(_) = &call.file //TODO: use .is_owned() when stabilized
            {
                Err(InvalidArg::InvalidFlag {
                    error: InvalidFlag::FileAlreadySpecified
                })
            }
            else
            {
                Ok(Err(Self::File))
            },
            FlagKind::Rev => Ok(Ok(Flag::Rev))
        }
    }

    fn parse(self, _no: usize, arg: String) -> Result<Result<Flag, Self>, Error>
    {
        match self
        {
            FlagCall::File => Ok(Ok(Flag::File(PathBuf::from(arg))))
        }
    }

    fn collect(self) -> Result<Option<Flag>, ExpectedArg>
    {
        match self
        {
            FlagCall::File => Err(ExpectedArg::Filename)
        }
    }

    fn replace_and_collect(self, replace: &mut Option<FlagCall>) -> Result<Option<Flag>, ExpectedArg>
    {
        if let Some(flag_call) = replace.replace(self) && let Some(flag) = flag_call.collect()?
        {
            Ok(Some(flag))
        }
        else
        {
            Ok(None)
        }
    }
}

pub struct Call
{
    exe: Option<&'static str>,
    flag_call: Option<FlagCall>,
    flags: Vec<FlagKind>,
    rounds: Rounds,
    file: Cow<'static, Path>,
    help: Option<Help>,
    rev: bool,
    no: usize
}

impl Call
{
    pub fn new() -> Self
    {
        Self {
            exe: None,
            flag_call: None,
            flags: Vec::new(),
            rounds: Rounds::All,
            file: Cow::Borrowed(crate::default_file_path()),
            help: None,
            rev: false,
            no: 0
        }
    }

    pub fn from_args(args: impl IntoIterator<Item = String>) -> Result<Self, Error>
    {
        let mut call = Self::new();
        call.parse(args)?;
        Ok(call)
    }

    fn next(&mut self, iter: &mut impl Iterator<Item = String>) -> Option<(&'static str, String)>
    {
        let exe = match self.exe
        {
            Some(exe) => exe,
            None => {
                self.exe = iter.next().map(|next| next.leak() as &str);
                self.exe?
            }
        };
        let next = iter.next();
        next.map(|next| {
            self.no += 1;

            (exe, next)
        })
    }

    fn add_flag(&mut self, flag: Flag) -> Result<(), InvalidFlag>
    {
        let kind = flag.kind();

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
            Flag::Rev => {
                self.rev ^= true;
                Ok(())
            }
        };

        Help::catch(add(), (), self.help.as_mut())?;
        self.flags.push(kind);

        Ok(())
    }

    fn add_flag_option(&mut self, flag: FlagOption<&str>, exe: &'static str) -> Result<(), ArgError>
    {
        if let Some(result) = Help::catch(FlagCall::new(flag, self, exe), None, self.help.as_mut())? && let Some(flag) = match result
        {
            Ok(flag) => Some(flag),
            Err(flag_call) => Help::catch(flag_call.replace_and_collect(&mut self.flag_call), None, self.help.as_mut())?,
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
            if let (Some(start), Some(end)) = (start, end) && start > end
            {
                return Err(InvalidArg::NotInOrder {
                    start: start + 1,
                    end: end + 1
                })
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

            if let Some(flag_call) = self.flag_call.take()
            {
                match flag_call.parse(self.no, arg.clone() /* :( */)?
                {
                    Ok(flag) => try_of!(self.add_flag(flag)),
                    Err(flag_call) => self.flag_call = Some(flag_call),
                };
                continue
            }

            fn parse_trimmed(call: &mut Call, exe: &'static str, arg: &str) -> Result<(), ArgError>
            {
                if let Some(flags) = arg.trim().strip_prefix("-")
                {
                    match flags.strip_prefix("-")
                    {
                        Some(flag) => call.add_flag_option(FlagOption::Long(flag), exe)?,
                        None => for flag in flags.chars()
                        {
                            call.add_flag_option(FlagOption::Short(flag), exe)?;
                        }
                    }

                    return Ok(())
                }
                else if call.help.is_some()
                {
                    return Err(InvalidArg::IntegerAfterHelp.into())
                }
                else
                {
                    call.rounds.add_round(Call::parse_round(arg)?);
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

        if let Some(flag_call) = self.flag_call.take() && let Some(flag) = Help::catch(flag_call.collect(), None, self.help.as_mut()).map_err(|e| e.at(exe, self.no))?
        {
            self.add_flag(flag).map_err(|e| e.at(exe, self.no, None))?;
        }

        let Self { exe: _, flag_call, mut flags, rounds, file, help, rev, no } = self;
        let _ = (flag_call, no);

        if let Some(mut help) = help
        {
            flags.retain(|flag| *flag != FlagKind::Help);
            help.prepend_flags(flags);
            return Err(Error::ShowHelp {
                help
            })
        }

        Run::new(rounds, file, rev)
    }
}