use anyhow::{anyhow, Context};

fn parse(input: &str) -> anyhow::Result<Vec<i32>> {
    input
        .trim()
        .split(',')
        .map(|l| {
            l.trim()
                .parse::<i32>()
                .with_context(|| format!("Can not parse {} to integer.", l))
        })
        .collect()
}

pub fn part1(text: &str) -> anyhow::Result<i32> {
    let mut input = parse(text)?;

    input.sort();

    let pos = input[input.len() / 2];

    let result = input.into_iter().map(|i| (i - pos).abs()).sum();
    Ok(result)
}

fn cost(start: i32, end: i32) -> i32 {
    let dist = (end - start).abs();
    dist * (dist + 1) / 2
}

fn eval_position(pos: i32, crabs: &[i32]) -> i32 {
    crabs
        .iter()
        .copied()
        .map(|crab_pos| cost(crab_pos, pos))
        .sum()
}

pub fn part2(text: &str) -> anyhow::Result<i32> {
    let mut input = parse(text)?;

    input.sort();

    let max = input.last().copied().ok_or(anyhow!("Input empty."))?;
    let min = input.first().copied().ok_or(anyhow!("Input empty."))?;

    let result = (min..max).map(|i| eval_position(i, &input)).min().unwrap();

    Ok(result)
}
