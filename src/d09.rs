use std::collections::{HashMap, VecDeque};

use anyhow::anyhow;

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

const NEIGHBORS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn neighbors(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    NEIGHBORS.into_iter().map(move |(dx, dy)| (x + dx, y + dy))
}

fn is_low_point(grid: &Grid<u32>, x: i32, y: i32) -> bool {
    let p = grid[(x, y)];

    neighbors(x, y).all(|(x, y)| p < *grid.get(x, y).unwrap_or(&u32::MAX))
}

fn get_low_points(grid: &Grid<u32>) -> impl Iterator<Item = (i32, i32, u32)> + '_ {
    grid.iter_coords::<i32>()
        .filter(move |&(x, y, _)| is_low_point(grid, x, y))
        .map(|(x, y, v)| (x, y, *v))
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    let grid = parse(text)?;

    let result = get_low_points(&grid).map(|(_, _, i)| i + 1).sum();

    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BasinID(usize);

#[allow(unused)]
fn print_basin(grid: &Grid<u32>, basin: &Grid<Option<BasinID>>) {
    use owo_colors::{OwoColorize, Style};

    let color = Style::new().green();
    let no_style = Style::new();

    let mut last_y = 0;
    for (x, y, &v) in grid.iter_coords::<i32>() {
        if last_y != y {
            println!();
            last_y = y;
        }

        let style = if basin[(x, y)].is_some() {
            color
        } else {
            no_style
        };

        print!("{}", v.to_string().style(style));
    }
    println!();
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    let grid = parse(text)?;

    let mut basins = Grid::new(grid.get_width(), grid.get_height(), None);

    let mut queue = VecDeque::new();

    for (id, (x, y, _)) in get_low_points(&grid).enumerate() {
        basins[(x, y)] = Some(BasinID(id));
        queue.push_back((x, y));
    }

    while !queue.is_empty() {
        let p @ (x, y) = queue.pop_front().unwrap();
        assert!(grid[p] < 9);

        let basin_id = basins[p];

        for p in neighbors(x, y).filter(|&(x, y)| grid.get(x, y).is_some()) {
            if grid[p] < 9 && basins[p].is_none() {
                basins[p] = basin_id;
                queue.push_back(p);
            }
        }
    }

    //print_basin(&grid, &basins);

    let mut basin_sizes = HashMap::new();

    for id in basins.iter().flat_map(|&i| i) {
        *basin_sizes.entry(id).or_insert(0) += 1;
    }

    let mut basin_sizes: Vec<_> = basin_sizes.values().copied().collect();

    basin_sizes.sort_by_key(|i| std::cmp::Reverse(*i));

    if basin_sizes.len() < 3 {
        return Err(anyhow!("Not enough basins for solution"));
    }

    Ok(basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
}
