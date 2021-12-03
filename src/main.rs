use aoc2021::{run_with_config, Input};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let input = Input::from_args();

    let result = run_with_config(&input)?;

    println!("{}", result);

    Ok(())
}
