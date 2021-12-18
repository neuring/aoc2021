use std::fmt;

use anyhow::anyhow;
use itertools::iproduct;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

type IResult<'src, T> = nom::IResult<&'src str, T>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SFNumber {
    Literal(u32),
    Pair(Box<SFNumber>, Box<SFNumber>),
}

impl SFNumber {
    fn add(self, other: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(other))
    }

    fn get_most_left_descdendent(&mut self) -> &mut u32 {
        match self {
            SFNumber::Literal(val) => val,
            SFNumber::Pair(l, _) => l.get_most_left_descdendent(),
        }
    }

    fn get_most_right_descdendent(&mut self) -> &mut u32 {
        match self {
            SFNumber::Literal(val) => val,
            SFNumber::Pair(_, r) => r.get_most_right_descdendent(),
        }
    }
}

impl fmt::Display for SFNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SFNumber::Literal(val) => write!(f, "{}", val),
            SFNumber::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
        }
    }
}

fn parse_literal(input: &str) -> IResult<'_, SFNumber> {
    digit1
        .map(|num: &str| SFNumber::Literal(num.parse::<u32>().unwrap()))
        .parse(input)
}

fn parse_pair(input: &str) -> IResult<'_, SFNumber> {
    tuple((
        tag("["),
        parse_sf_number,
        tag(","),
        parse_sf_number,
        tag("]"),
    ))
    .map(|(_, lhs, _, rhs, _)| SFNumber::Pair(Box::new(lhs), Box::new(rhs)))
    .parse(input)
}

fn parse_sf_number(input: &str) -> IResult<'_, SFNumber> {
    alt((parse_literal, parse_pair)).parse(input)
}

fn parse_all_sf_numbers(input: &str) -> IResult<'_, Vec<SFNumber>> {
    separated_list1(multispace1, parse_sf_number).parse(input)
}

fn parse(input: &str) -> anyhow::Result<Vec<SFNumber>> {
    parse_all_sf_numbers(input)
        .map_err(|_| anyhow!("Invalid input"))
        .map(|(_, result)| result)
}

fn explode(
    sf: &mut SFNumber,
    depth: u32,
    left_neighbor: Option<&mut SFNumber>,
    right_neighbor: Option<&mut SFNumber>,
) -> bool {
    use SFNumber::*;

    match sf {
        Literal(_) => false,

        Pair(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
            (Literal(left), Literal(right)) if depth >= 4 => {
                if let Some(l) = left_neighbor {
                    let l = l.get_most_right_descdendent();
                    *l += left;
                }
                if let Some(r) = right_neighbor {
                    let r = r.get_most_left_descdendent();
                    *r += right;
                }

                let _ = std::mem::replace(sf, Literal(0));
                true
            }
            _ => {
                explode(lhs, depth + 1, left_neighbor, Some(rhs))
                    || explode(rhs, depth + 1, Some(lhs), right_neighbor)
            }
        },
    }
}

fn split(sf: &mut SFNumber) -> bool {
    match sf {
        SFNumber::Literal(val) if *val >= 10 => {
            let lower = *val / 2;
            let upper = *val / 2 + if *val & 1 != 0 { 1 } else { 0 };
            let new_pair = SFNumber::Pair(
                Box::new(SFNumber::Literal(lower)),
                Box::new(SFNumber::Literal(upper)),
            );
            let _ = std::mem::replace(sf, new_pair);
            true
        }
        SFNumber::Literal(_) => false,
        SFNumber::Pair(lhs, rhs) => split(lhs) || split(rhs),
    }
}

fn reduce(sf: &mut SFNumber) {
    while explode(sf, 0, None, None) || split(sf) {}
}

fn magnitude(sf: &SFNumber) -> u32 {
    match sf {
        SFNumber::Literal(value) => *value,
        SFNumber::Pair(lhs, rhs) => 3 * magnitude(lhs) + 2 * magnitude(rhs),
    }
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    let input = parse(text)?;
    let sum = input
        .into_iter()
        .reduce(|acc, item| {
            let mut sum = acc.add(item);
            reduce(&mut sum);
            sum
        })
        .expect("Parse would have failed, if empty");
    Ok(magnitude(&sum))
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    let input = parse(text)?;

    let result = iproduct!(input.iter().enumerate(), input.iter().enumerate())
        .filter(|((i1, _), (i2, _))| i1 != i2)
        .map(|((_, l), (_, r))| (l, r))
        .map(|(l, r)| {
            let mut sum = l.clone().add(r.clone());
            reduce(&mut sum);
            magnitude(&sum)
        })
        .max()
        .unwrap();
    Ok(result)
}
