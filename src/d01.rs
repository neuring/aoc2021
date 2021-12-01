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

pub fn entry(input: String, second_puzzle: bool) -> anyhow::Result<()> {
    let mut data = parse(&input)?;

    if second_puzzle {
        data = data.array_windows().map(|&[a, b, c]| a + b + c).collect();
    }

    let result = data.array_windows().filter(|&[a, b]| a < b).count();

    if !second_puzzle {
        println!("Increasing {} times", result);
    } else {
        println!("Sliding sum is increasing {} times", result);
    }

    Ok(())
}
