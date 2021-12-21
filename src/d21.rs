use std::lazy::SyncLazy;

use anyhow::anyhow;
use regex::Regex;

fn parse(input: &str) -> anyhow::Result<[u64; 2]> {
    static INPUT_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
        Regex::new(r"Player (?P<player>\d+) starting position: (?P<start>\d+)")
            .unwrap()
    });

    let mut result = [0; 2];

    for line in input.trim().lines() {
        let cap = INPUT_REGEX
            .captures(line.trim())
            .ok_or(anyhow!("Invalid input '{}'", line.trim()))?;

        let player = cap["player"].parse::<usize>().unwrap();
        let start = cap["start"].parse::<u64>().unwrap();
        result[player - 1] = start;
    }

    Ok(result)
}

struct DeterministicDice(u64);

impl DeterministicDice {
    fn new() -> Self {
        Self(1)
    }

    fn next(&mut self) -> u64 {
        let result = self.0;
        self.0 = mod1(self.0 + 1, 100);
        result
    }
}

fn mod1(a: u64, m: u64) -> u64 {
    (a - 1) % m + 1
}

pub fn part1(text: &str) -> anyhow::Result<u64> {
    let mut positions = parse(text)?;

    let mut scores = [0; 2];

    let mut dice = DeterministicDice::new();

    let mut total_dice_throws: u64 = 0;

    for dice_throw in 0.. {
        let player = dice_throw % 2;

        let dice_result = dice.next() + dice.next() + dice.next();

        positions[player] = mod1(positions[player] + dice_result, 10);
        scores[player] += positions[player];

        if scores[player] >= 1000 {
            total_dice_throws = 3 * (dice_throw as u64 + 1);
            break;
        }
    }

    Ok(total_dice_throws * scores.iter().min().unwrap())
}

fn step(
    timelines: u64,
    turn: usize,
    pos: u64,
    score: u64,
    finishes: &mut [u64],
    ts: &mut [u64],
) {
    assert!(score < 21);

    ts[turn] += timelines;

    for (dice, multiplier) in
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
    {
        let new_pos = mod1(pos + dice, 10);
        let new_score = score + new_pos;
        let new_timelines = multiplier * timelines;

        if new_score >= 21 {
            finishes[turn] += new_timelines;
        } else {
            step(new_timelines, turn + 1, new_pos, new_score, finishes, ts)
        }
    }
}

struct FinishResult {
    finishes: Vec<u64>,
    timelines: Vec<u64>,
}

fn collect_finishes(pos: u64) -> FinishResult {
    let mut result = vec![0; 21];
    let mut timelines = vec![0; 21];

    step(1, 0, pos, 0, &mut result, &mut timelines);

    FinishResult {
        finishes: result,
        timelines,
    }
}

pub fn part2(text: &str) -> anyhow::Result<u64> {
    let positions = parse(text)?;

    let FinishResult {
        finishes: f1,
        timelines: t1,
    } = collect_finishes(positions[0]);

    let FinishResult {
        finishes: f2,
        timelines: t2,
    } = collect_finishes(positions[1]);

    let mut player1_wins = 0;
    for (i, &w) in f1.iter().enumerate() {
        player1_wins += w * t2[i];
    }

    let mut player2_wins = 0;
    for (i, &w) in f2[..20].iter().enumerate() {
        player2_wins += w * t1[i + 1];
    }

    Ok(u64::max(player1_wins, player2_wins))
}
