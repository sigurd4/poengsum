use flag_example::FlagExample;
use flags_examples::FlagsExamples;

use crate::flag::FlagKind;

moddef::moddef!(
    mod {
        flags_examples,
        flag_example
    }
);

pub struct Help;

impl Help
{
    pub fn flags_examples() -> FlagsExamples
    {
        FlagsExamples
    }
    pub fn flag_example(flag: FlagKind) -> FlagExample
    {
        FlagExample(flag)
    }
}