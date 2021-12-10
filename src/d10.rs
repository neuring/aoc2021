use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BracketType {
    Normal,
    Square,
    Curly,
    Angled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bracket {
    ty: BracketType,
    dir: Direction,
}

fn parse_bracket(c: char) -> anyhow::Result<Bracket> {
    use BracketType::*;
    use Direction::*;
    #[rustfmt::skip]
    let b = match c {
        '(' => Bracket { ty: Normal, dir: Open },
        ')' => Bracket { ty: Normal, dir: Close },
        '[' => Bracket { ty: Square, dir: Open },
        ']' => Bracket { ty: Square, dir: Close },
        '{' => Bracket { ty: Curly,  dir: Open },
        '}' => Bracket { ty: Curly,  dir: Close },
        '<' => Bracket { ty: Angled, dir: Open },
        '>' => Bracket { ty: Angled, dir: Close },
        _ => bail!("'{}' is not a bracket", c)
    };
    Ok(b)
}

fn parse(input: &str) -> anyhow::Result<Vec<Vec<Bracket>>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(parse_bracket)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

#[derive(Debug)]
enum VerifyResult {
    /// Successfully verified
    Ok,

    /// Invalid syntax at position.
    Illegal(usize),

    /// Syntax is correct but missing tokens.
    Incomplete { expected_tokens: Vec<BracketType> },
}

fn verify_line(input: &[Bracket]) -> VerifyResult {
    let mut stack = Vec::with_capacity(input.len() / 2);

    for (idx, &b) in input.iter().enumerate() {
        match b.dir {
            Direction::Open => stack.push(b.ty),
            Direction::Close => {
                if let Some(&top) = stack.last() {
                    if b.ty == top {
                        stack.pop();
                    } else {
                        return VerifyResult::Illegal(idx);
                    }
                } else {
                    return VerifyResult::Illegal(idx);
                }
            }
        }
    }

    if !stack.is_empty() {
        return VerifyResult::Incomplete {
            expected_tokens: stack,
        };
    }

    VerifyResult::Ok
}

pub fn part1(text: &str) -> anyhow::Result<u64> {
    let input = parse(text)?;

    let mut score = 0;

    for line in input {
        match verify_line(&line) {
            VerifyResult::Illegal(pos) => {
                score += match line[pos].ty {
                    BracketType::Normal => 3,
                    BracketType::Square => 57,
                    BracketType::Curly => 1197,
                    BracketType::Angled => 25137,
                };
            }
            VerifyResult::Incomplete { .. } | VerifyResult::Ok => {}
        }
    }

    Ok(score)
}

pub fn part2(text: &str) -> anyhow::Result<u64> {
    let input = parse(text)?;

    let mut scores: Vec<_> = input
        .iter()
        .map(|line| verify_line(&line))
        .filter_map(|v| {
            if let VerifyResult::Incomplete { expected_tokens } = v {
                Some(expected_tokens)
            } else {
                None
            }
        })
        .map(|mut stack| {
            stack.reverse();
            stack
                .into_iter()
                .map(|t| match t {
                    BracketType::Normal => 1,
                    BracketType::Square => 2,
                    BracketType::Curly => 3,
                    BracketType::Angled => 4,
                })
                .fold(0, |acc, n| acc * 5 + n)
        })
        .collect();

    scores.sort();

    let score = scores[scores.len() / 2];

    Ok(score)
}
