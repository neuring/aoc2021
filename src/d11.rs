use std::collections::{BTreeSet, HashSet};

use anyhow::anyhow;
use itertools::iproduct;

use crate::grid::Grid;

fn parse(input: &str) -> anyhow::Result<Grid<u32>> {
    let height = input.trim().lines().count();
    let data = input
        .trim()
        .lines()
        .flat_map(|line| {
            line.trim().chars().map(|digit| {
                digit
                    .to_digit(10)
                    .ok_or(anyhow!("'{}' is not a digit.", digit))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let width = data.len() / height;

    Ok(Grid::from_rows_columns(width, height, data))
}

fn neighbors(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    iproduct!(-1..=1, -1..=1)
        .filter(|&(x, y)| x != 0 || y != 0)
        .map(move |(dx, dy)| (x + dx, y + dy))
}

/// Simulates a step of octopi flashing.
/// Returns the positions of all discharged octopi.
fn simulate_step(
    grid: &mut Grid<u32>,
    worklist: &mut BTreeSet<(i32, i32)>,
) -> HashSet<(i32, i32)> {
    assert!(worklist.is_empty());

    // Increase every octopus level and remember in worklist which will flash.
    for (x, y, v) in grid.iter_coords_mut() {
        *v += 1;

        if *v > 9 {
            worklist.insert((x, y));
        }
    }

    let mut flashes = HashSet::new();

    // For every flash, remember who flashed and increase level of surrounding octopi.
    // If a surrounding octopus has reached a sufficiently high level to flash, append it to
    // the worklist.
    while let Some((x, y)) = worklist.pop_first() {
        assert!(grid[(x, y)] > 9);

        grid[(x, y)] -= 10;
        assert!(flashes.insert((x, y)), "duplicate flash at ({}, {})", x, y);

        for (nx, ny) in neighbors(x, y) {
            if let Some(v) = grid.get_mut(nx, ny) {
                *v += 1;

                if *v > 9 {
                    worklist.insert((nx, ny));
                }
            }
        }
        //println!("flash at ({}, {})", x, y);
        //print_grid(&grid, &[(x, y)].into_iter().collect());
    }

    // After a step every flashed octopus has to be at level zero.
    for &(x, y) in &flashes {
        grid[(x, y)] = 0;
    }

    flashes
}

#[allow(unused)]
fn print_grid(grid: &Grid<u32>, flashes: &HashSet<(i32, i32)>) {
    use owo_colors::{OwoColorize, Style};

    let bold = Style::new().white().bold();
    let normal = Style::new().white();
    let get_style =
        |x, y| flashes.contains(&(x, y)).then_some(bold).unwrap_or(normal);

    let mut last_y = 0;
    for (x, y, &v) in grid.iter_coords::<i32>() {
        if last_y != y {
            println!();
            last_y = y;
        }

        print!("{}", v.to_string().style(get_style(x, y)));
    }
    println!();
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    let mut input = parse(text)?;
    let mut worklist = BTreeSet::new();

    let mut total_flashes = 0;
    for _ in 1..=100 {
        let flashes = simulate_step(&mut input, &mut worklist);

        total_flashes += flashes.len();
    }

    Ok(total_flashes)
}

pub fn part2(text: &str) -> anyhow::Result<i32> {
    let mut input = parse(text)?;
    let mut worklist = BTreeSet::new();

    for i in 1.. {
        let flashes = simulate_step(&mut input, &mut worklist);

        if flashes.len() == input.get_height() * input.get_width() {
            return Ok(i);
        }
    }

    unreachable!();
}
