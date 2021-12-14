use std::collections::HashMap;

use anyhow::{anyhow, ensure};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Element(char);

#[derive(Debug)]
struct ParseResult {
    starting_sequence: Vec<Element>,
    rules: HashMap<[Element; 2], Element>,
}

fn parse_rule(input: &str) -> anyhow::Result<([Element; 2], Element)> {
    let mut parts = input.trim().split("->");

    let pair = parts
        .next()
        .ok_or(anyhow!("Missing starting pair in rule."))?;

    let mut elements = pair.trim().chars().map(Element);
    let first_element = elements.next().ok_or(anyhow!("Missing starting pair"))?;
    let second_element = elements
        .next()
        .ok_or(anyhow!("Missing second element in starting pair"))?;
    ensure!(
        elements.next().is_none(),
        "Too many starting elements in rule."
    );

    let inserted_element = parts
        .next()
        .ok_or(anyhow!("Missing insertion element in rule."))?
        .trim();
    ensure!(
        inserted_element.chars().count() == 1,
        "There can only be a single insertion element per rule."
    );
    let inserted_element = Element(inserted_element.chars().next().unwrap());

    Ok(([first_element, second_element], inserted_element))
}

fn parse(input: &str) -> anyhow::Result<ParseResult> {
    let mut parts = input.trim().split("\n\n");

    let sequence = parts.next().ok_or(anyhow!("Can't find input sequence"))?;
    let sequence = sequence.trim().chars().map(Element).collect();

    let rules = parts.next().ok_or(anyhow!("Can't find insertion rules."))?;

    let rules = rules
        .trim()
        .lines()
        .map(parse_rule)
        .collect::<Result<_, _>>()?;

    Ok(ParseResult {
        starting_sequence: sequence,
        rules,
    })
}

type ElementCount = HashMap<Element, u64>;

fn combine_count(map: &mut ElementCount, other: &ElementCount) {
    for (&key, &value) in other {
        *map.entry(key).or_insert(0) += value;
    }
}

fn solve(input: &str, iterations: u64) -> anyhow::Result<u64> {
    let ParseResult {
        starting_sequence,
        rules,
    } = parse(input)?;

    // table for dynamic programming.
    // stores the element count if you expand an element pair for n iterations.
    // (excluding the original two elements themselves)
    let mut table: HashMap<([Element; 2], u64), ElementCount> = HashMap::new();

    // initialize table
    for pair in rules.keys() {
        table.insert((*pair, 0), HashMap::new());
    }

    // Calculate the number of inserted elements between a pair.
    // Let's say the current pair is AB and C would be inserted between them.
    // For iteration i, we get the the inserted elements for AC and AB with i-1 iterations
    // Add them together and, finally, add the C element.
    for iteration in 1..=iterations {
        for (&[first, second], &inserted_element) in &rules {
            let mut count =
                table[&([first, inserted_element], iteration - 1)].clone();

            combine_count(
                &mut count,
                &table[&([inserted_element, second], iteration - 1)],
            );

            *count.entry(inserted_element).or_insert(0) += 1;

            table.insert(([first, second], iteration), count);
        }
    }

    let mut result = ElementCount::new();

    // Accumulate all *inserted* elements.
    for elem in starting_sequence.array_windows::<2>() {
        combine_count(&mut result, &table[&(*elem, iterations)]);
    }

    // Add the sequence elementes themselves.
    for elem in starting_sequence {
        *result.entry(elem).or_insert(0) += 1;
    }

    // Calculate result
    let max = result.values().copied().max().unwrap();
    let min = result.values().copied().min().unwrap();

    Ok(max - min)
}

pub fn part1(text: &str) -> anyhow::Result<u64> {
    solve(text, 10)
}

pub fn part2(text: &str) -> anyhow::Result<u64> {
    solve(text, 40)
}
