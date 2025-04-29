use std::path::PathBuf;

use crate::help::Help;

moddef::moddef!(
    flat(pub) mod {
        flag_kind,
        flag_option
    }
);

pub enum Flag
{
    Help(Help),
    File(PathBuf),
    Rev,
}

impl Flag
{
    pub const VARIANT_COUNT: usize = 2; //core::mem::variant_count::<FlagKind>(); TODO use this instead when stable
    pub const VARIANTS: [FlagKind; Self::VARIANT_COUNT] = [FlagKind::Help, FlagKind::File];

    pub fn kind(&self) -> FlagKind
    {
        match self
        {
            Self::Help(..) => FlagKind::Help,
            Self::File(..) => FlagKind::File,
            Self::Rev => FlagKind::Rev
        }
    }
}