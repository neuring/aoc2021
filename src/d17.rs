use std::lazy::SyncLazy;

use anyhow::anyhow;
use itertools::iproduct;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, x: i32) -> bool {
        self.start <= x && x <= self.end
    }
}

impl IntoIterator for Range {
    type Item = i32;

    type IntoIter = impl Iterator<Item = i32> + Clone;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

#[derive(Debug)]
struct TargetArea {
    x_range: Range,
    y_range: Range,
}

impl TargetArea {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.x_range.contains(x) && self.y_range.contains(y)
    }
}

fn parse(input: &str) -> anyhow::Result<TargetArea> {
    static INPUT_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
        Regex::new(r"target area: x=(?P<x_start>-?\d+)..(?P<x_end>-?\d+), y=(?P<y_start>-?\d+)..(?P<y_end>-?\d+)").unwrap()
    });

    let cap = INPUT_REGEX
        .captures(input.trim())
        .ok_or(anyhow!("Invalid input format"))?;

    let x_start = &cap["x_start"];
    let x_start = x_start
        .parse::<i32>()
        .map_err(|_| anyhow!("x start value is not a number: '{}'", x_start))?;
    let x_end = &cap["x_end"];
    let x_end = x_end
        .parse::<i32>()
        .map_err(|_| anyhow!("x end value is not a number: '{}'", x_end))?;
    let y_start = &cap["y_start"];
    let y_start = y_start
        .parse::<i32>()
        .map_err(|_| anyhow!("y start value is not a number: '{}'", y_start))?;
    let y_end = &cap["y_end"];
    let y_end = y_end
        .parse::<i32>()
        .map_err(|_| anyhow!("y end value is not a number: '{}'", y_end))?;

    Ok(TargetArea {
        x_range: Range {
            start: x_start,
            end: x_end,
        },
        y_range: Range {
            start: y_start,
            end: y_end,
        },
    })
}

pub fn part1(text: &str) -> anyhow::Result<i32> {
    let input = parse(text)?;

    let max_speed = max_y_start_vel(input.y_range);

    let height = max_speed * (max_speed + 1) / 2;

    Ok(height)
}

fn max_y_start_vel(y_target_range: Range) -> i32 {
    let y_start = y_target_range.start;

    assert!(y_start < 0);

    -y_start - 1
}

fn get_x_start_vel_range(target_x_range: Range) -> Range {
    let mut x = 0;

    for i in 1.. {
        if x + i > target_x_range.start {
            return Range {
                start: i,
                end: target_x_range.end,
            };
        }

        x += i;
    }

    unreachable!();
}

fn get_y_start_vel_range(target_y_range: Range) -> Range {
    Range {
        start: target_y_range.start,
        end: max_y_start_vel(target_y_range),
    }
}

fn simulate_shot(mut x_vel: i32, mut y_vel: i32, target: &TargetArea) -> bool {
    let mut pos_x = 0;
    let mut pos_y = 0;

    loop {
        if pos_x > target.x_range.end || pos_y < target.y_range.start {
            return false;
        }

        if target.contains(pos_x, pos_y) {
            return true;
        }

        pos_x += x_vel;
        pos_y += y_vel;
        x_vel = i32::max(0, x_vel - 1);
        y_vel -= 1;
    }
}

pub fn part2(text: &str) -> anyhow::Result<usize> {
    let area @ TargetArea { x_range, y_range } = parse(text)?;

    let start_x_vel_range = get_x_start_vel_range(x_range);
    let start_y_vel_range = get_y_start_vel_range(y_range);

    let shots = iproduct!(start_x_vel_range, start_y_vel_range)
        .filter(|&(x, y)| simulate_shot(x, y, &area));

    let n = shots.count();

    Ok(n)
}
