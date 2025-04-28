use core::fmt::Display;

use super::{ArgUsage, FlagUsage, FlagsUsages, Help, Rules};

pub enum Docs<'a>
{
    ArgUsage(ArgUsage),
    Help(&'a Help),
    //FlagHelp(FlagHelp),
    FlagsUsages(FlagsUsages),
    FlagUsage(FlagUsage),
    Rules(Rules)
}

impl Display for Docs<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Docs::ArgUsage(arg_usage) => arg_usage.fmt(f),
            Docs::Help(help) => help.fmt(f),
            //Docs::FlagHelp(flag_help) => flag_help.fmt(f),
            Docs::FlagsUsages(flags_usages) => flags_usages.fmt(f),
            Docs::FlagUsage(flag_usage) => flag_usage.fmt(f),
            Docs::Rules(rules) => rules.fmt(f),
        }
    }
}