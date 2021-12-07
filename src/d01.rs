use anyhow::anyhow;

fn parse(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .split_whitespace()
        .map(|i| {
            i.parse::<i64>()
                .map_err(|_| anyhow!("{} is not a number.", i))
        })
        .collect::<Result<Vec<_>, _>>()
}

fn entry(input: &str, second_puzzle: bool) -> anyhow::Result<usize> {
    let mut data = parse(&input)?;

    if second_puzzle {
        data = data.array_windows().map(|&[a, b, c]| a + b + c).collect();
    }

    let result = data.array_windows().filter(|&[a, b]| a < b).count();

    Ok(result)
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    entry(text, false)
}

pub fn part2(text: &str) -> anyhow::Result<usize> {
    entry(text, true)
}
