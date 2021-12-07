#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(drain_filter)]
#![feature(let_else)]
#![feature(bool_to_option)]

use std::{fmt, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail};

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Day(u32);

impl Day {
    pub fn new(day: u32) -> Self {
        if day > 25 {
            panic!("Invalid day")
        } else {
            Day(day)
        }
    }
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

        if day > 25 {
            return Err(DayParseError::InvalidDay(day));
        }

        Ok(Day(day))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Puzzle {
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

fn select(input: &Input, text: &str) -> anyhow::Result<String> {
    #[allow(unreachable_patterns)]
    let result = match (input.day, input.puzzle) {
        (Day(01), Puzzle::First) => d01::part1(text)?.to_string(),
        (Day(01), Puzzle::Second) => d01::part2(text)?.to_string(),
        (Day(02), Puzzle::First) => d02::part1(text)?.to_string(),
        (Day(02), Puzzle::Second) => d02::part2(text)?.to_string(),
        (Day(03), Puzzle::First) => d03::part1(text)?.to_string(),
        (Day(03), Puzzle::Second) => d03::part2(text)?.to_string(),
        (Day(04), Puzzle::First) => d04::part1(text)?.to_string(),
        (Day(04), Puzzle::Second) => d04::part2(text)?.to_string(),
        (Day(05), Puzzle::First) => d05::part1(text)?.to_string(),
        (Day(05), Puzzle::Second) => d05::part2(text)?.to_string(),
        (Day(06), Puzzle::First) => d06::part1(text)?.to_string(),
        (Day(06), Puzzle::Second) => d06::part2(text)?.to_string(),
        (Day(07), Puzzle::First) => d07::part1(text)?.to_string(),
        (Day(07), Puzzle::Second) => d07::part2(text)?.to_string(),
        _ => bail!("Not implemented!"),
    };

    Ok(result)
}

#[derive(structopt::StructOpt)]
pub struct Input {
    pub input: PathBuf,

    #[structopt(short, long)]
    pub day: Day,

    #[structopt(default_value, short, long)]
    pub puzzle: Puzzle,
}

pub fn run_with_config(input: &Input) -> anyhow::Result<String> {
    let text = std::fs::read_to_string(&input.input)?;

    let result = select(input, &text)?;
    Ok(result)
}
