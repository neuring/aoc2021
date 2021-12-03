#![feature(type_alias_impl_trait)]
#![feature(array_windows)]

use std::{fmt, path::PathBuf, str::FromStr};

use anyhow::anyhow;
use structopt::StructOpt;

mod d01;
mod d02;
mod d03;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Day {
    D01,
    D02,
    D03,
}

#[derive(Debug, thiserror::Error)]
enum DayParseError {
    #[error("Input '{0}' is not a number.")]
    NotANumber(String),

    #[error("There is (currently?) not a {0} day.")]
    InvalidDay(u32),
}

impl FromStr for Day {
    type Err = impl std::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day: u32 = s
            .parse()
            .map_err(|_| DayParseError::NotANumber(s.to_string()))?;

        let day = match day {
            1 => Day::D01,
            2 => Day::D02,
            3 => Day::D03,
            _ => return Err(DayParseError::InvalidDay(day)),
        };

        Ok(day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Puzzle {
    First,
    Second,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Puzzle::First => write!(f, "First"),
            Puzzle::Second => write!(f, "Second"),
        }
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::First
    }
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ["first", "First", "1", "one"].contains(&s) {
            Ok(Puzzle::First)
        } else if ["second", "Second", "2", "two"].contains(&s) {
            Ok(Puzzle::Second)
        } else {
            Err(anyhow!("{} is an invalid puzzle selection.", s))
        }
    }
}

fn select(input: Input, text: &str) -> anyhow::Result<()> {
    #[allow(unreachable_patterns)]
    match (input.day, input.puzzle) {
        (Day::D01, Puzzle::First) => d01::entry(text, false),
        (Day::D01, Puzzle::Second) => d01::entry(text, true),
        (Day::D02, Puzzle::First) => d02::entry1(text),
        (Day::D02, Puzzle::Second) => d02::entry2(text),
        (Day::D03, Puzzle::First) => d03::entry1(text),
        (Day::D03, Puzzle::Second) => d03::entry2(text),
        _ => Err(anyhow!("Not implemented!")),
    }
}

#[derive(structopt::StructOpt)]
struct Input {
    input: PathBuf,

    #[structopt(short, long)]
    day: Day,

    #[structopt(default_value, short, long)]
    puzzle: Puzzle,
}

fn main() -> anyhow::Result<()> {
    let input = Input::from_args();

    let text = std::fs::read_to_string(&input.input)?;

    select(input, &text)
}
