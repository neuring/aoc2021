use anyhow::{anyhow, bail};

enum Direction {
    Up,
    Down,
    Forward,
}

impl Direction {
    fn get_xy_dir(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Forward => (1, 0),
        }
    }
}

struct Instruction {
    direction: Direction,
    length: u32,
}

fn parse(text: &str) -> anyhow::Result<Vec<Instruction>> {
    text.trim()
        .split('\n')
        .map(|i| {
            let mut instruction = i.trim().split_whitespace();

            let direction =
                instruction.next().ok_or(anyhow!("Missing direction."))?;
            let direction = match direction {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => bail!("Invalid direction '{}'", direction),
            };

            let length = instruction.next().ok_or(anyhow!("Missing distance"))?;
            let length = length.parse()?;

            Ok(Instruction { direction, length })
        })
        .collect()
}

pub fn part1(text: &str) -> anyhow::Result<i32> {
    let instructions = parse(&text)?;

    let (x, y) = instructions
        .iter()
        .map(|i| {
            let (x, y) = i.direction.get_xy_dir();
            let length = i.length as i32;
            (x * length, y * length)
        })
        .fold((0, 0), |(x, y), (nx, ny)| (x + nx, y + ny));

    Ok(x * y)
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    let instructions = parse(&text)?;

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for instr in instructions {
        match instr.direction {
            Direction::Up => aim -= instr.length,
            Direction::Down => aim += instr.length,
            Direction::Forward => {
                x += instr.length;
                y += aim * instr.length;
            }
        }
    }

    Ok(x * y)
}
