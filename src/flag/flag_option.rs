use core::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Debug)]
pub enum FlagOption<S>
where
    S: Deref<Target = str>
{
    Long(S),
    Short(char)
}
impl From<FlagOption<&str>> for FlagOption<Box<str>>
{
    fn from(opt: FlagOption<&str>) -> Self
    {
        opt.into()
    }
}
impl<S> FlagOption<S>
where
    S: Deref<Target = str>
{
    pub fn into<T>(self) -> FlagOption<T>
    where
        T: Deref<Target = str>,
        S: Into<T>
    {
        match self
        {
            Self::Long(opt) => FlagOption::Long(opt.into()),
            Self::Short(opt) => FlagOption::Short(opt)
        }
    }
}

impl<S> Display for FlagOption<S>
where
    S: Deref<Target = str>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            FlagOption::Long(opt) => opt.fmt(f),
            FlagOption::Short(opt) => opt.fmt(f),
        }
    }
}