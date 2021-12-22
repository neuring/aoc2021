use std::{collections::HashSet, lazy::SyncLazy};

use ahash::AHashSet;
use anyhow::anyhow;
use itertools::iproduct;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl IntoIterator for Range {
    type Item = i64;

    type IntoIter = impl Iterator<Item = i64> + Clone;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Cuboid {
    state: State,
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn is_small(&self) -> bool {
        -50 <= self.x.start
            && self.x.end <= 50
            && -50 <= self.y.start
            && self.y.end <= 50
            && -50 <= self.z.start
            && self.z.end <= 50
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Cuboid>> {
    static INPUT_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
        Regex::new(r"(?P<state>on|off) x=(?P<x_start>-?\d+)..(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)..(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)..(?P<z_end>-?\d+)").unwrap()
    });

    input
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<Cuboid> {
            let caps = INPUT_REGEX
                .captures(line.trim())
                .ok_or(anyhow!("Illegal input"))?;

            let state = match &caps["state"] {
                "on" => State::On,
                "off" => State::Off,
                _ => unreachable!("Regex would be invalid"),
            };

            let x_start = caps["x_start"]
                .parse::<i64>()
                .map_err(|_| anyhow!("Illegal x start"))?;
            let x_end = caps["x_end"]
                .parse::<i64>()
                .map_err(|_| anyhow!("Illegal x end"))?;
            let y_start = caps["y_start"]
                .parse::<i64>()
                .map_err(|_| anyhow!("Illegal y start"))?;
            let y_end = caps["y_end"]
                .parse::<i64>()
                .map_err(|_| anyhow!("Illegal y end"))?;
            let z_start = caps["z_start"]
                .parse::<i64>()
                .map_err(|_| anyhow!("Illegal z start"))?;
            let z_end = caps["z_end"]
                .parse::<i64>()
                .map_err(|_| anyhow!("Illegal z end"))?;

            #[rustfmt::skip]
            let result = Cuboid {
                state,
                x: Range { start: x_start, end: x_end },
                y: Range { start: y_start, end: y_end },
                z: Range { start: z_start, end: z_end },
            };
            Ok(result)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
    z: T,
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    let input = parse(text)?;

    let input: Vec<_> = input.into_iter().filter(Cuboid::is_small).collect();

    let mut enabled = HashSet::new();

    for cuboid in input {
        for p in iproduct!(cuboid.x, cuboid.y, cuboid.z).map(|(x, y, z)| Point {
            x,
            y,
            z,
        }) {
            match cuboid.state {
                State::On => enabled.insert(p),
                State::Off => enabled.remove(&p),
            };
        }
    }

    Ok(enabled.len())
}

fn get_space_range(
    range: &Range,
    space: &[i64],
) -> impl Iterator<Item = u16> + Clone {
    let start = space.binary_search(&range.start).unwrap();
    let end = space.binary_search(&range.end);
    let end = match end {
        Ok(i) => i + 1,
        Err(i) => i,
    };

    let start: u16 = start.try_into().unwrap();
    let end: u16 = end.try_into().unwrap();

    start..end
}

pub fn part2(text: &str) -> anyhow::Result<i64> {
    let input = parse(text)?;

    let mut x_space: Vec<_> = input
        .iter()
        .flat_map(|c| [c.x.start, c.x.end + 1])
        .collect();
    x_space.sort();
    x_space.dedup();

    let mut y_space: Vec<_> = input
        .iter()
        .flat_map(|c| [c.y.start, c.y.end + 1])
        .collect();
    y_space.sort();
    y_space.dedup();

    let mut z_space: Vec<_> = input
        .iter()
        .flat_map(|c| [c.z.start, c.z.end + 1])
        .collect();
    z_space.sort();
    z_space.dedup();

    println!("Created x,y,z spaces");

    let mut enabled: AHashSet<_> = AHashSet::new();

    for (i, cuboid) in input.into_iter().enumerate() {
        println!("cuboid: {}", i);
        for p in iproduct!(
            get_space_range(&cuboid.x, &x_space),
            get_space_range(&cuboid.y, &y_space),
            get_space_range(&cuboid.z, &z_space)
        )
        .map(|(x, y, z)| Point { x, y, z })
        {
            match cuboid.state {
                State::On => enabled.insert(p),
                State::Off => enabled.remove(&p),
            };
        }
    }

    println!("processed cuboids: {}", enabled.len());

    let sum = enabled
        .into_iter()
        .map(|p| {
            let x = p.x as usize;
            let y = p.y as usize;
            let z = p.z as usize;
            let x = x_space[x + 1] - x_space[x];
            let y = y_space[y + 1] - y_space[y];
            let z = z_space[z + 1] - z_space[z];
            x * y * z
        })
        .sum();

    println!("finished summing");

    Ok(sum)
}
