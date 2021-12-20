use anyhow::bail;
use itertools::iproduct;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Dark,
    Light,
}

struct InfiniteGrid {
    inner: Grid<Pixel>,
    surrounding: Pixel,
}

struct ParseResult {
    sequence: Vec<Pixel>,
    input: InfiniteGrid,
}

fn parse_grid(grid: &str) -> anyhow::Result<Grid<Pixel>> {
    let height = grid.trim().lines().count();
    let width = grid.trim().lines().next().unwrap().trim().chars().count();

    let grid: Result<_, _> = grid
        .trim()
        .lines()
        .flat_map(|l| l.trim().chars())
        .map(|c| match c {
            '.' => Ok(Pixel::Dark),
            '#' => Ok(Pixel::Light),
            _ => bail!("Illegal pixel {}", c),
        })
        .collect();

    Ok(Grid::from_rows_columns(width, height, grid?))
}

fn parse(input: &str) -> anyhow::Result<ParseResult> {
    let mut parts = input.trim().split("\n\n");

    let sequence = parts.next().unwrap();
    let sequence: Vec<_> = sequence
        .trim()
        .chars()
        .map(|c| match c {
            '.' => Ok(Pixel::Dark),
            '#' => Ok(Pixel::Light),
            _ => bail!("Illegal pixel {}", c),
        })
        .collect::<Result<_, _>>()?;

    assert_eq!(sequence.len(), 512);

    let image = parts.next().unwrap();
    let image = parse_grid(image)?;

    Ok(ParseResult {
        sequence,
        input: InfiniteGrid {
            inner: image,
            surrounding: Pixel::Dark,
        },
    })
}

fn neighbors(x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
    iproduct!(-1..=1, -1..=1).map(move |(dy, dx)| (x + dx, y + dy))
}

fn enhance(image: &InfiniteGrid, sequence: &[Pixel]) -> InfiniteGrid {
    let new_width = image.inner.get_width() + 2;
    let new_height = image.inner.get_height() + 2;

    let mut new_image = Grid::new(new_width, new_height, Pixel::Dark);

    for (x, y, p) in new_image.iter_coords_mut::<i32>() {
        let mut sequence_idx = 0;

        for p in neighbors(x, y).map(|(x, y)| {
            let old_img_x = x - 1;
            let old_img_y = y - 1;
            *image
                .inner
                .get(old_img_x, old_img_y)
                .unwrap_or(&image.surrounding)
        }) {
            sequence_idx <<= 1;
            if p == Pixel::Light {
                sequence_idx |= 1;
            }
        }

        *p = sequence[sequence_idx];
    }

    let new_surrounding = if image.surrounding == Pixel::Light {
        sequence.last().unwrap()
    } else {
        sequence.first().unwrap()
    };

    InfiniteGrid {
        inner: new_image,
        surrounding: *new_surrounding,
    }
}

#[allow(unused)]
fn print_image(grid: &Grid<Pixel>) {
    for line in grid.rows() {
        for &b in line {
            print!("{}", if b == Pixel::Light { '#' } else { '.' });
        }
        println!();
    }
}

fn solve(text: &str, iterations: usize) -> anyhow::Result<usize> {
    let ParseResult {
        sequence,
        mut input,
    } = parse(text)?;

    for _ in 0..iterations {
        input = enhance(&input, &sequence);
    }

    assert_eq!(input.surrounding, Pixel::Dark);

    let result = input.inner.iter().filter(|&&p| p == Pixel::Light).count();

    Ok(result)
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    solve(text, 2)
}

pub fn part2(text: &str) -> anyhow::Result<usize> {
    solve(text, 50)
}
