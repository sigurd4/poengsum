use std::path::PathBuf;

pub enum Flag
{
    File(PathBuf)
}

impl Flag
{
    pub const VARIANT_COUNT: usize = 1;
    pub const OPTIONS: [&str; Self::VARIANT_COUNT] = [
        "file"
    ];
}