use core::fmt::Display;

use crate::{error::{ArgError, ErrorMsg, ExpectedArg, InvalidArg, InvalidFlag}, flag::FlagKind};

moddef::moddef!(
    flat(pub) mod {
        arg_usage,
        call_example,
        line_example,
        flag_help,
        flag_usage,
        flags_usages,
        rules,
        docs
    }
);

const ALIGN: usize = 8*4;

#[derive(Clone, Debug)]
pub struct Help
{
    exe: &'static str,
    flags: Vec<FlagKind>
}

pub enum Catch
{
    Args,
    Flag(FlagKind)
}

pub trait HelpCatchable: Into<ArgError>
{
    fn catch(self) -> Result<Catch, Self>;
}

impl HelpCatchable for InvalidFlag
{
    fn catch(self) -> Result<Catch, Self>
    {
        Ok(Catch::Flag(self.related_flag()))
    }
}
impl HelpCatchable for InvalidArg
{
    fn catch(self) -> Result<Catch, Self>
    {
        match self
        {
            InvalidArg::NonexistentFlag { .. }
            | InvalidArg::IntegerAfterHelp => Err(self),

            InvalidArg::UseTwoDots
            | InvalidArg::CannotParseInteger { .. }
            | InvalidArg::NotInOrder { .. }
            | InvalidArg::RoundZero => Ok(Catch::Args),

            InvalidArg::InvalidFlag { error } => Ok(Catch::Flag(error.related_flag()))
        }
    }
}
impl HelpCatchable for ExpectedArg
{
    fn catch(self) -> Result<Catch, Self>
    {
        match self
        {
            ExpectedArg::Filename => Ok(Catch::Flag(FlagKind::File)),
        }
    }
}

impl Help
{
    pub fn new(exe: &'static str) -> Self
    {
        Self {
            exe,
            flags: Vec::new()
        }
    }

    pub fn catch<T, U, E>(result: Result<T, E>, or: U, help: Option<&mut Self>) -> Result<U, E>
    where
        E: HelpCatchable,
        T: Into<U>
    {
        crate::catch(result, or, |error| {
            match help
            {
                Some(help) => {
                    help.reap_catch(error.catch()?);
                    Ok(())
                },
                None => Err(error)
            }
        })
    }

    pub fn reap_catch(&mut self, catch: Catch)
    {
        match catch
        {
            Catch::Args => (),
            Catch::Flag(flag) => self.add_flag(flag),
        }
    }

    pub fn and(&mut self, help: Help)
    {
        let Help {exe, flags} = help;

        assert_eq!(self.exe, exe, "How did this happen?");

        self.add_flag(FlagKind::Help);

        for flag in flags
        {
            self.add_flag(flag);
        }
    }

    pub fn add_flag(&mut self, flag: FlagKind)
    {
        if !self.flags.contains(&flag)
        {
            self.flags.push(flag);
        }
    }

    pub fn prepend_flags(&mut self, mut flags: Vec<FlagKind>)
    {
        std::mem::swap(&mut self.flags, &mut flags);
        for flag in flags
        {
            self.add_flag(flag);
        }
    }
}

impl Display for Help
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut flags = self.flags.iter();
        if let Some(&flag) = flags.next()
        {
            let help = FlagHelp {
                exe: &*self.exe,
                flag
            };

            write!(f, "{help}")?;

            for &flag in flags
            {
                let help = FlagHelp {
                    exe: &*self.exe,
                    flag
                };

                write!(f, "\n{help}")?;
            }

            return Ok(())
        }

        let arg_usage = ArgUsage {
            exe: &*self.exe
        };
        let flags_usage = FlagsUsages {
            exe: &*self.exe
        };
        write!(f, "{arg_usage}\n\n{flags_usage}")
    }
}

impl Help
{
    pub fn msg(&self) -> ErrorMsg
    {
        let mut msg = "Showing help".to_string();
        let mut flags = self.flags.iter();

        if let Some(mut flag) = flags.next()
        {
            let mut msgs = vec![msg];
            msgs.push(format!(" for \"--{flag}\""));

            if let Some(next) = flags.next()
            {
                flag = next;
                for next in flags
                {
                    msgs.push(format!(", \"--{flag}\""));
                    flag = next;
                }

                msgs.push(format!(" and \"--{flag}\"\n"));
            }

            msg = msgs.concat()
        }

        ErrorMsg {
            msg: (msg + ".").into_boxed_str(),
            error: None,
            line: None,
            hint: None,
            docs: Some(Docs::Help(self))
        }
    }
}