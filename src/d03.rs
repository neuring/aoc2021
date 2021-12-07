use anyhow::{anyhow, bail};

fn bin_str_to_u32(s: &str) -> anyhow::Result<u32> {
    let mut result = 0;
    for c in s.chars() {
        match c {
            '0' => {
                result <<= 1;
            }
            '1' => {
                result += 1;
                result <<= 1;
            }
            _ => bail!("Invalid character '{}' in binary string.", c),
        }
    }
    result >>= 1;
    Ok(result)
}

pub fn get_value_width(text: &str) -> anyhow::Result<usize> {
    let width = text
        .trim()
        .split_whitespace()
        .next()
        .ok_or(anyhow!("No data in file"))?
        .chars()
        .count();
    Ok(width)
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    let value_width = get_value_width(text)?;

    let mut num_total_values = 0;
    let mut digits = vec![0; value_width];

    for num in text.trim().split_whitespace().map(bin_str_to_u32) {
        let mut num = num?;

        for i in (0..value_width).rev() {
            digits[i] += if num & 1 == 1 { 1 } else { 0 };
            num >>= 1;
        }

        num_total_values += 1;
    }

    let mut result = 0;

    for i in digits {
        result |= (2 * i > num_total_values) as u32;
        result <<= 1;
    }

    result >>= 1;

    let gamma_rate = result;
    let epsilon_rate = !result & !(u32::MAX << value_width);

    Ok(gamma_rate * epsilon_rate)
}

fn filter_value_with_bit(data: &mut Vec<u32>, bit: usize, bit_is_one: bool) {
    let cmp_fn = if bit_is_one { |x| x > 0 } else { |x| x == 0 };

    data.retain(|&v| cmp_fn(v & (1 << bit)));
}

fn find_oxygen_rating(
    mut data: Vec<u32>,
    value_width: usize,
) -> anyhow::Result<u32> {
    let mut current_bit = value_width - 1;

    loop {
        let data_len = data.len();

        let counted_ones = data
            .iter()
            .filter(|&&value| value & (1 << current_bit) > 0)
            .count();

        if 2 * counted_ones >= data_len {
            filter_value_with_bit(&mut data, current_bit, true);
        } else {
            filter_value_with_bit(&mut data, current_bit, false);
        }

        if data.len() == 1 {
            break;
        }

        current_bit = current_bit
            .checked_sub(1)
            .ok_or(anyhow!("No unique solution found."))?;
    }

    if data.is_empty() {
        bail!("No solution found");
    }

    Ok(data[0])
}

fn find_scrubber_rating(
    mut data: Vec<u32>,
    value_width: usize,
) -> anyhow::Result<u32> {
    let mut current_bit = value_width - 1;

    loop {
        let data_len = data.len();

        let counted_zeroes = data
            .iter()
            .filter(|&&value| value & (1 << current_bit) == 0)
            .count();

        if 2 * counted_zeroes > data_len {
            filter_value_with_bit(&mut data, current_bit, true);
        } else {
            filter_value_with_bit(&mut data, current_bit, false);
        }

        if data.len() == 1 {
            break;
        }

        current_bit = current_bit
            .checked_sub(1)
            .ok_or(anyhow!("No unique solution found."))?;
    }

    if data.is_empty() {
        bail!("No solution found");
    }

    Ok(data[0])
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    let value_width = get_value_width(text)?;

    let data = text
        .trim()
        .split_whitespace()
        .map(bin_str_to_u32)
        .collect::<Result<Vec<_>, _>>()?;

    let oxygen_rating = find_oxygen_rating(data.clone(), value_width)?;
    let scrubber_rating = find_scrubber_rating(data, value_width)?;

    Ok(oxygen_rating * scrubber_rating)
}
