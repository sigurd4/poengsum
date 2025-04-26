#![feature(iterator_try_collect)]
#![feature(unsigned_signed_diff)]

use error::Error;
use record::Record;
use score::Score;

moddef::moddef!(
    flat mod {
        error,
        record,
        score
    }
);

const POENGSUM_PATH: &str = "./poengsum.txt";

fn main()
{
    if let Err(error) = run(std::env::args())
    {
        eprintln!("Error: {error:?}\n\n{error}")
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Error>
{
    let records = Record::read()?;

    let scores = Score::scores(records, parse_args(args)?)?;

    Score::present(scores);

    Ok(())
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Option<Vec<usize>>, Error>
{
    let _ = args.next();

    let rounds = args
        .enumerate()
        .map(|(i, arg)| {
            let no = i + 1;
            arg.parse::<usize>()
                .map_err(|error| Error::CannotParseRound { no, arg, error })?
                .checked_sub(1)
                .ok_or(Error::RoundZero { no })
        })
        .try_collect::<Vec<_>>()?;

    Ok(if !rounds.is_empty() { Some(rounds) } else { None })
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
