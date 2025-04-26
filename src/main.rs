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

const POENGSUM_PATH: &'static str = "./poengsum.txt";

fn main()
{
    if let Err(error) = run()
    {
        eprintln!("Error: {0:?}\n\n{0}", error)
    }
}

fn run() -> Result<(), Error>
{
    let records = Record::read()?;

    let scores = Score::scores(records, parse_args()?)?;

    Score::present(scores);

    Ok(())
}

fn parse_args() -> Result<Option<Vec<usize>>,  Error>
{
    let mut args = std::env::args();
    let _ = args.next();

    let rounds = args.into_iter()
        .enumerate()
        .map(|(i, arg)| {
            let no = i + 1;
            arg.parse::<usize>()
                .map_err(|error| Error::CannotParseRound {
                    no,
                    arg,
                    error
                })?
                .checked_sub(1)
                .ok_or(Error::RoundZero {
                    no
                })
        }).try_collect::<Vec<_>>()?;

    Ok(
        if !rounds.is_empty()
        {
            Some(rounds)
        }
        else
        {
            None
        }
    )
}