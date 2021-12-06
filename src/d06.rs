use std::collections::VecDeque;

use anyhow::Context;

fn parse(input: &str) -> anyhow::Result<Vec<usize>> {
    input
        .trim()
        .split(',')
        .map(|l| {
            l.trim()
                .parse::<usize>()
                .with_context(|| format!("Can not parse {} to integer.", l))
        })
        .collect()
}

fn simulate(input: &str, days: usize) -> anyhow::Result<usize> {
    let input = parse(input)?;

    let mut data = VecDeque::new();
    data.resize(9, 0);

    for i in input {
        data[i] += 1;
    }

    for _day in 0..days {
        let f = data.pop_front().unwrap();
        data.push_back(f);
        data[6] += f;
    }

    Ok(data.into_iter().sum::<usize>())
}

pub fn entry1(text: &str) -> anyhow::Result<usize> {
    simulate(text, 80)
}

pub fn entry2(text: &str) -> anyhow::Result<usize> {
    simulate(text, 256)
}
