#[derive(Debug)]
pub enum InvalidIO
{
    Open,
    Read {
        row: usize
    }
}