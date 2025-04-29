use core::fmt::Display;

use crate::style;

use super::ALIGN;

pub struct CallExample
{
    pub exe: &'static str,
    pub args: Vec<Box<str>>,
    pub effect: Option<Box<str>>
}

impl CallExample
{
    #[cfg(test)]
    pub fn into_args(self) -> impl Iterator<Item = String>
    {
        let Self {exe, args, effect: _} = self;

        core::iter::once(exe.to_string())
            .chain(args.into_iter()
                .map(|arg| arg.into_string())
            )
    }
}

impl Display for CallExample
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut align = ALIGN;

        let exe = style::exe(self.exe);
        align = align.saturating_sub(1 + exe.len());
        write!(f, "{exe}")?;

        for arg in self.args.iter()
        {
            let arg = style::arg(&**arg);
            align = align.saturating_sub(1 + arg.len());
            write!(f, " {arg}")?;
        }

        if let Some(effect) = self.effect.as_ref()
        {
            let effect = style::info(&**effect);
            write!(f, "{empty:>align$} {effect}", empty = "")?;
        }

        Ok(())
    }
}