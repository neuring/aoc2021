use std::collections::HashSet;

use anyhow::{anyhow, bail, ensure, Context};

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Fold {
    X(i32),
    Y(i32),
}

struct ParseResult {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

fn parse_point(input: &str) -> anyhow::Result<Point> {
    let mut parts = input.split(',');
    let x = parts.next().ok_or(anyhow!("Missing x coordinate"))?;
    let x: i32 = x
        .parse()
        .map_err(|_| anyhow!("Invalid x coordinate value '{}'", x))?;

    let y = parts.next().ok_or(anyhow!("Missing y coordinate"))?;
    let y: i32 = y
        .parse()
        .map_err(|_| anyhow!("Invalid y coordinate value '{}'", y))?;

    Ok(Point { x, y })
}

fn parse_fold(input: &str) -> anyhow::Result<Fold> {
    let mut parts = input.trim().split_whitespace();

    ensure!(
        parts.next().map(|f| f == "fold").unwrap_or(false),
        "Missing fold keyword"
    );
    ensure!(
        parts.next().map(|f| f == "along").unwrap_or(false),
        "Missing along keyword"
    );

    let actual_fold = parts.next().ok_or(anyhow!("Missing fold"))?;

    let mut fold = actual_fold.chars();
    let dimension = fold.next().unwrap();

    ensure!(
        fold.next().map(|f| f == '=').unwrap_or(false),
        "Missing equal sign"
    );

    let value: i32 = fold
        .as_str()
        .parse()
        .map_err(|_| anyhow!("Invalid value '{}'", fold.as_str()))?;

    let fold = match dimension {
        'x' => Fold::X(value),
        'y' => Fold::Y(value),
        _ => bail!("Invalid dimensional axis '{}'", dimension),
    };

    Ok(fold)
}

fn parse(input: &str) -> anyhow::Result<ParseResult> {
    let mut parts = input.trim().split("\n\n");

    let points = parts.next().ok_or(anyhow!("Missing points"))?;

    let points = points
        .trim()
        .lines()
        .enumerate()
        .map(|(n, line)| {
            parse_point(line)
                .with_context(|| format!("Invalid point '{}' at {}.", line, n))
        })
        .collect::<Result<_, _>>()?;

    let folds = parts.next().ok_or(anyhow!("Missing fold instructions"))?;
    let folds = folds
        .trim()
        .lines()
        .map(|l| parse_fold(l))
        .collect::<Result<_, _>>()?;

    Ok(ParseResult { points, folds })
}

fn apply_fold(points: &HashSet<Point>, f: Fold) -> HashSet<Point> {
    #[rustfmt::skip]
    let func = |&p: &Point| -> Point {
        match f {
            Fold::X(v) => Point { x: if p.x > v { 2*v - p.x } else { p.x }, ..p },
            Fold::Y(v) => Point { y: if p.y > v { 2*v - p.y } else { p.y }, ..p },
        }
    };

    points.iter().map(func).collect()
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    let ParseResult { points, folds } = parse(text)?;

    let folded_points = apply_fold(&points, folds[0]);

    Ok(folded_points.len())
}

pub fn part2(text: &str) -> anyhow::Result<String> {
    let ParseResult { mut points, folds } = parse(text)?;

    for fold in folds {
        points = apply_fold(&points, fold);
    }

    let x_max = points
        .iter()
        .map(|&Point { x, .. }| x)
        .max()
        .ok_or(anyhow!("Missing points"))?;
    let y_max = points
        .iter()
        .map(|&Point { y, .. }| y)
        .max()
        .ok_or(anyhow!("Missing points"))?;

    assert!(x_max >= 0);
    assert!(y_max >= 0);

    let mut grid = Grid::new(x_max as usize + 1, y_max as usize + 1, false);
    for Point { x, y } in points {
        grid[(x, y)] = true;
    }

    let result = itertools::join(
        grid.rows().map(|row| {
            row.iter()
                .map(|&t| if t { '#' } else { '.' })
                .collect::<String>()
        }),
        "\n",
    );

    Ok(result)
}
