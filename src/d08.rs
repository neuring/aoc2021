use std::collections::BTreeSet;

use anyhow::{anyhow, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

type Digit = BTreeSet<Segment>;

#[derive(Debug)]
struct InputLine {
    digits: Vec<Digit>,
    sequence: Vec<Digit>,
}

fn parse_digit(digit: &str) -> anyhow::Result<Digit> {
    let set = digit
        .trim()
        .chars()
        .map(|c| {
            Ok(match c {
                'a' => Segment::A,
                'b' => Segment::B,
                'c' => Segment::C,
                'd' => Segment::D,
                'e' => Segment::E,
                'f' => Segment::F,
                'g' => Segment::G,
                _ => bail!("Invalid character '{}'", c),
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(set)
}

fn parse(input: &str) -> anyhow::Result<Vec<InputLine>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let digits = parts.next().ok_or(anyhow!("Missing digits"))?;

            let digits = digits
                .trim()
                .split_whitespace()
                .map(parse_digit)
                .collect::<Result<_, _>>()?;

            let sequence = parts.next().ok_or(anyhow!("Missing sequence"))?;
            let sequence = sequence
                .trim()
                .split_whitespace()
                .map(parse_digit)
                .collect::<Result<_, _>>()?;

            Ok(InputLine { digits, sequence })
        })
        .collect()
}

fn decode_line(
    InputLine { digits, sequence }: InputLine,
) -> anyhow::Result<Vec<usize>> {
    // These four digits are uniquely identifiable
    let one_digit = digits
        .iter()
        .filter(|b| b.len() == 2)
        .next()
        .ok_or(anyhow!("No two segment digit."))?
        .clone();

    let four_digit = digits
        .iter()
        .filter(|b| b.len() == 4)
        .next()
        .ok_or(anyhow!("No four segment digit."))?;

    let seven_digit = digits
        .iter()
        .filter(|b| b.len() == 3)
        .next()
        .ok_or(anyhow!("No three segment digit."))?
        .clone();

    let eight_digit = digits
        .iter()
        .filter(|b| b.len() == 7)
        .next()
        .ok_or(anyhow!("No seven segment digit."))?
        .clone();

    // A segment is the difference in segments of the digits seven and one.
    let a_segment = seven_digit.difference(&one_digit).copied().next().unwrap();

    // By subtracting one from digit four, we are left with B and D segments, which, allows us
    // To identify the zero digit because it is the only digit with 6 segments that only looses
    // one segment when both b and d are subtracted.
    let b_and_d_segments: Digit =
        four_digit.difference(&one_digit).copied().collect();

    let s: Digit = eight_digit.difference(&b_and_d_segments).copied().collect();

    let zero_digit: Vec<_> =
        digits.iter().filter(|b| b.len() == 6).cloned().collect();
    let mut zero_digit: Vec<_> = zero_digit
        .into_iter()
        .filter(|b| b.difference(&s).count() == 1)
        .collect();
    assert_eq!(zero_digit.len(), 1);
    let zero_digit: Digit = zero_digit.pop().unwrap();

    // With the zero digit we can get D by subtracting it from digit 8, which then can be removed
    // from b_and_d_segments to get d.
    let mut d_segment: Vec<_> =
        eight_digit.difference(&zero_digit).copied().collect();
    assert_eq!(d_segment.len(), 1);
    let d_segment = d_segment.pop().unwrap();

    let mut b_segment: Vec<_> = b_and_d_segments
        .iter()
        .filter(|&&s| s != d_segment)
        .copied()
        .collect();
    assert_eq!(b_segment.len(), 1);
    let b_segment = b_segment.pop().unwrap();

    // We can identify the five digit be cause it is the only five segment digit
    // Which has two segments left, when we subtract a, b and d, which leaves us
    // with f and g.
    let a_b_d_segments: Digit =
        [a_segment, b_segment, d_segment].into_iter().collect();

    let f_and_g_segments: Vec<_> = digits
        .iter()
        .filter_map(|b| {
            if b.len() != 5 {
                return None;
            }
            let diff: Digit = b.difference(&a_b_d_segments).copied().collect();
            (diff.len() == 2).then_some(diff)
        })
        .collect();
    assert_eq!(f_and_g_segments.len(), 1);
    let f_or_g_segments: Digit = f_and_g_segments[0].clone();
    assert_eq!(f_or_g_segments.len(), 2);

    // The intersection of {f, c} and {f, g} is f, which then allows us to extract c and g.
    let mut f_segment: Vec<_> =
        one_digit.intersection(&f_or_g_segments).copied().collect();
    assert_eq!(f_segment.len(), 1);
    let f_segment = f_segment.pop().unwrap();

    let g_segment = f_or_g_segments
        .iter()
        .filter(|&&b| b != f_segment)
        .copied()
        .next()
        .unwrap();

    let c_segment = one_digit
        .iter()
        .filter(|&&b| b != f_segment)
        .copied()
        .next()
        .unwrap();

    // The only digit left is e, which we get by subtracting the segments of nine from digit eight.
    let nine_digit: Digit = vec![
        a_segment, b_segment, c_segment, d_segment, f_segment, g_segment,
    ]
    .into_iter()
    .collect();

    let mut e_segment: Vec<_> =
        eight_digit.difference(&nine_digit).copied().collect();
    assert_eq!(e_segment.len(), 1);
    let e_segment = e_segment.pop().unwrap();

    // We have identified all digits, so now we're constructing all remaining digits.
    let zero_digit: Digit = vec![
        a_segment, b_segment, c_segment, e_segment, f_segment, g_segment,
    ]
    .into_iter()
    .collect();

    let two_digit: Digit =
        vec![a_segment, c_segment, d_segment, e_segment, g_segment]
            .into_iter()
            .collect();

    let three_digit: Digit =
        vec![a_segment, c_segment, d_segment, f_segment, g_segment]
            .into_iter()
            .collect();

    let four_digit: Digit = vec![b_segment, c_segment, d_segment, f_segment]
        .into_iter()
        .collect();

    let five_digit: Digit =
        vec![a_segment, b_segment, d_segment, f_segment, g_segment]
            .into_iter()
            .collect();

    let six_digit: Digit = vec![
        a_segment, b_segment, d_segment, e_segment, f_segment, g_segment,
    ]
    .into_iter()
    .collect();

    let digits = vec![
        zero_digit,
        one_digit,
        two_digit,
        three_digit,
        four_digit,
        five_digit,
        six_digit,
        seven_digit,
        eight_digit,
        nine_digit,
    ];

    // With all digits we can now decode every digit and return.

    let decode_digit =
        |d: &Digit| digits.iter().enumerate().find(|(_, o)| &d == o).unwrap().0;

    let solution = sequence.iter().map(decode_digit).collect();
    Ok(solution)
}

pub fn part1(text: &str) -> anyhow::Result<usize> {
    let input = parse(text)?;

    let mut result = 0;

    for line in input {
        let digits = decode_line(line)?;

        result += digits.iter().filter(|&i| [1, 7, 4, 8].contains(i)).count();
    }

    Ok(result)
}

pub fn part2(text: &str) -> anyhow::Result<usize> {
    let input = parse(text)?;

    let mut result = 0;

    for line in input {
        let digits = decode_line(line)?;

        let mut s = 0;
        for d in digits {
            s *= 10;
            s += d;
        }

        result += s;
    }

    Ok(result)
}
