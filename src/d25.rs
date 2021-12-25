use anyhow::{bail, ensure};

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Right,
    Down,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Right => '>',
            Tile::Down => 'v',
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Grid<Tile>> {
    let height = input.trim().lines().count();
    ensure!(height > 0, "Grid is empty");
    let width = input.trim().lines().next().unwrap().trim().chars().count();

    let grid: Result<_, _> = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().chars())
        .map(|c| match c {
            '.' => Ok(Tile::Empty),
            '>' => Ok(Tile::Right),
            'v' => Ok(Tile::Down),
            _ => bail!("Illegal tile '{}'", c),
        })
        .collect();

    let grid = grid?;

    Ok(Grid::from_rows_columns(width, height, grid))
}

struct State {
    current: Grid<Tile>,
    scratch: Grid<Tile>,
}

impl State {
    fn new(start: Grid<Tile>) -> Self {
        let scratch = start.clone();
        Self {
            current: start,
            scratch,
        }
    }

    fn update(&mut self) -> bool {
        let mut changed = false;

        self.scratch.fill(Tile::Empty);

        // Move cucumbers right.
        for (x, y, t) in self.current.iter_coords::<i32>() {
            match t {
                Tile::Right => {
                    let next_x = (x + 1) % (self.current.get_width() as i32);

                    let real_x;
                    if self.current[(next_x, y)] == Tile::Empty {
                        real_x = next_x;
                        changed = true;
                    } else {
                        real_x = x;
                    }

                    self.scratch[(real_x, y)] = Tile::Right;
                }
                Tile::Down => self.scratch[(x, y)] = *t,
                Tile::Empty => {}
            }
        }

        std::mem::swap(&mut self.current, &mut self.scratch);
        //print_grid(&self.current);

        self.scratch.fill(Tile::Empty);

        // Move cucumbers down.
        for (x, y, t) in self.current.iter_coords::<i32>() {
            match t {
                Tile::Down => {
                    let next_y = (y + 1) % (self.current.get_height() as i32);

                    let real_y;
                    if self.current[(x, next_y)] == Tile::Empty {
                        real_y = next_y;
                        changed = true;
                    } else {
                        real_y = y;
                    }

                    self.scratch[(x, real_y)] = Tile::Down;
                }
                Tile::Right => self.scratch[(x, y)] = *t,
                Tile::Empty => {}
            }
        }

        std::mem::swap(&mut self.current, &mut self.scratch);

        changed
    }
}

#[allow(unused)]
fn print_grid(grid: &Grid<Tile>) {
    for row in grid.rows() {
        for b in row {
            print!("{}", b.to_char());
        }
        println!()
    }
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    let grid = parse(text)?;

    let mut state = State::new(grid);

    let mut counter = 0;

    while state.update() {
        counter += 1;
    }

    Ok(counter + 1)
}
