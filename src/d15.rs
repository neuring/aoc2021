use std::collections::BinaryHeap;

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

#[derive(Debug, Eq, Ord)]
struct HeapEntry {
    coord: (i32, i32),
    score: u32,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // We want a min heap, so we reverse the ordering.
        Some(self.score.cmp(&other.score).reverse())
    }
}

const NEIGHBORS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn neighbors((x, y): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    NEIGHBORS.into_iter().map(move |(dx, dy)| (x + dx, y + dy))
}

// dijkstras algorithm on a 2D grid
fn shortest_path(grid: &Grid<u32>) -> anyhow::Result<u32> {
    let mut scores = Grid::new(grid.get_width(), grid.get_height(), None);

    let mut queue = BinaryHeap::new();
    queue.push(HeapEntry {
        coord: (0, 0),
        score: 0,
    });
    scores[(0, 0)] = Some(0);

    // We want to get to the bottom right.
    let end_coord = (grid.get_width() as i32 - 1, grid.get_height() as i32 - 1);

    while let Some(HeapEntry { coord, score }) = queue.pop() {
        if coord == end_coord {
            break;
        }

        for (neighbor_coord, _) in neighbors(coord)
            .filter_map(|(x, y)| grid.get(x, y).map(|&s| ((x, y), s)))
        {
            let new_neighbor_score = score + grid[neighbor_coord];

            let is_better_score = scores[neighbor_coord]
                .filter(|&current_score| current_score <= new_neighbor_score)
                .is_none();
            if is_better_score {
                scores[neighbor_coord] = Some(new_neighbor_score);
                queue.push(HeapEntry {
                    coord: neighbor_coord,
                    score: new_neighbor_score,
                });
            }
        }
    }

    Ok(scores[end_coord].unwrap())
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    let grid = parse(text)?;
    shortest_path(&grid)
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    let small_grid = parse(text)?;
    let mut larger_grid =
        Grid::new(small_grid.get_width() * 5, small_grid.get_height() * 5, 0);

    for (gx, gy) in iproduct!(0..5, 0..5) {
        for (x, y, score) in small_grid.iter_coords::<i32>() {
            let coord_in_large_grid = (
                x + gx * small_grid.get_width() as i32,
                y + gy * small_grid.get_height() as i32,
            );

            larger_grid[coord_in_large_grid] =
                (score + gx as u32 + gy as u32 - 1) % 9 + 1;
        }
    }

    shortest_path(&larger_grid)
}
