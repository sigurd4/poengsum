use std::path::Path;

use error::Error;

moddef::moddef!(
    flat mod {
        call,
        run,
        error,
        flag,
        record,
        round,
        score
    }
);

// TODO: use .try_collect from std once stabilized
fn try_collect<C, T, E>(iter: &mut impl Iterator<Item = Result<T, E>>) -> Result<C, E>
where
    C: FromIterator<T>
{
    let mut err = None;

    let collection = core::iter::repeat_with(|| iter.next())
        .map_while(|x| x.and_then(|x| x.map_err(|e| err = Some(e)).ok()))
        .collect();

    if let Some(err) = err
    {
        return Err(err);
    }

    Ok(collection)
}

// TODO: use .checked_signed_diff from std once stabilized
fn checked_signed_diff(lhs: usize, rhs: usize) -> Option<isize>
{
    let res = lhs.wrapping_sub(rhs) as isize;
    let overflow = (lhs >= rhs) == (res < 0);

    if !overflow { Some(res) } else { None }
}

fn default_file_path() -> &'static Path
{
    Path::new("./poengsum.txt")
}

fn main()
{
    if let Err(error) = run(std::env::args())
    {
        eprintln!("{error}")
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Error>
{
    RunBuilder::from_args(args)?
        .collect()?
        .scores()?
        .present();

    Ok(())
}

#[cfg(test)]
mod tests
{
    use crate::error::Error;

    #[test]
    fn it_works() -> Result<(), Error>
    {
        crate::run(["poengsum"].into_iter().map(String::from))?;

        crate::run(["poengsum", "1"].into_iter().map(String::from))?;
        crate::run(["poengsum", "2"].into_iter().map(String::from))?;
        crate::run(["poengsum", "3"].into_iter().map(String::from))?;

        crate::run(["poengsum", "1", "2"].into_iter().map(String::from))?;
        crate::run(["poengsum", "2", "3"].into_iter().map(String::from))?;

        crate::run(["poengsum", "1", "2", "3"].into_iter().map(String::from))?;

        Ok(())
    }
}
