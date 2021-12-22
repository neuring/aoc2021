#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(drain_filter)]
#![feature(let_else)]
#![feature(bool_to_option)]
#![feature(map_first_last)]
#![feature(once_cell)]

use std::{fmt, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail};

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod graph;
mod grid;

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
        (Day(08), Puzzle::First) => d08::part1(text)?.to_string(),
        (Day(08), Puzzle::Second) => d08::part2(text)?.to_string(),
        (Day(09), Puzzle::First) => d09::part1(text)?.to_string(),
        (Day(09), Puzzle::Second) => d09::part2(text)?.to_string(),
        (Day(10), Puzzle::First) => d10::part1(text)?.to_string(),
        (Day(10), Puzzle::Second) => d10::part2(text)?.to_string(),
        (Day(11), Puzzle::First) => d11::part1(text)?.to_string(),
        (Day(11), Puzzle::Second) => d11::part2(text)?.to_string(),
        (Day(12), Puzzle::First) => d12::part1(text)?.to_string(),
        (Day(12), Puzzle::Second) => d12::part2(text)?.to_string(),
        (Day(13), Puzzle::First) => d13::part1(text)?.to_string(),
        (Day(13), Puzzle::Second) => d13::part2(text)?.to_string(),
        (Day(14), Puzzle::First) => d14::part1(text)?.to_string(),
        (Day(14), Puzzle::Second) => d14::part2(text)?.to_string(),
        (Day(15), Puzzle::First) => d15::part1(text)?.to_string(),
        (Day(15), Puzzle::Second) => d15::part2(text)?.to_string(),
        (Day(16), Puzzle::First) => d16::part1(text)?.to_string(),
        (Day(16), Puzzle::Second) => d16::part2(text)?.to_string(),
        (Day(17), Puzzle::First) => d17::part1(text)?.to_string(),
        (Day(17), Puzzle::Second) => d17::part2(text)?.to_string(),
        (Day(18), Puzzle::First) => d18::part1(text)?.to_string(),
        (Day(18), Puzzle::Second) => d18::part2(text)?.to_string(),
        (Day(19), Puzzle::First) => d19::part1(text)?.to_string(),
        (Day(19), Puzzle::Second) => d19::part2(text)?.to_string(),
        (Day(20), Puzzle::First) => d20::part1(text)?.to_string(),
        (Day(20), Puzzle::Second) => d20::part2(text)?.to_string(),
        (Day(21), Puzzle::First) => d21::part1(text)?.to_string(),
        (Day(21), Puzzle::Second) => d21::part2(text)?.to_string(),
        (Day(22), Puzzle::First) => d22::part1(text)?.to_string(),
        (Day(22), Puzzle::Second) => d22::part2(text)?.to_string(),
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
