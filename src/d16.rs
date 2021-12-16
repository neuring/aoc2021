use bitvec::prelude::*;
use itertools::Itertools;

#[derive(Debug)]
enum PacketType {
    Literal(u64),
    Operator {
        op: PacketOp,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug)]
struct Packet {
    version: u32,
    packet_type: PacketType,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketOp {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

fn parse_packet_op(val: u32) -> PacketOp {
    use PacketOp::*;

    match val {
        0 => Sum,
        1 => Product,
        2 => Min,
        3 => Max,
        5 => GreaterThan,
        6 => LessThan,
        7 => Equal,
        _ => unreachable!(),
    }
}

fn parse_value(bits: &BitSlice<Msb0, u8>) -> u32 {
    let mut v = 0;

    for i in 0..bits.len() {
        if bits[i] {
            v |= 1 << (bits.len() - 1 - i);
        }
    }

    v
}

fn parse_literal_packet(
    mut bits: &BitSlice<Msb0, u8>,
) -> (PacketType, &BitSlice<Msb0, u8>) {
    let mut result: u64 = 0;
    let result_view: &mut BitSlice<Msb0, u64> = result.view_bits_mut();
    let mut result_size = 0;

    while bits[0] {
        result_view[result_size..result_size + 4].clone_from_bitslice(&bits[1..5]);
        result_size += 4;
        bits = &bits[5..];
    }

    result_view[result_size..result_size + 4].clone_from_bitslice(&bits[1..5]);
    result_size += 4;
    bits = &bits[5..];

    result >>= u64::BITS as usize - result_size;

    (PacketType::Literal(result), bits)
}

fn parse_operator_packet(
    bits: &BitSlice<Msb0, u8>,
    op_id: u32,
) -> (PacketType, &BitSlice<Msb0, u8>) {
    let length_type_id = bits[0] as u32;

    let mut subpackets = Vec::new();

    let remaining_bits;

    if length_type_id == 0 {
        let length_of_subpackets = parse_value(&bits[1..16]);

        let mut bits = &bits[16..];

        let remaining_size = bits.len();

        assert!(remaining_size >= length_of_subpackets as _);

        while remaining_size - bits.len() < length_of_subpackets as usize {
            let (packet, remaining) = parse_packet(bits);
            bits = remaining;
            subpackets.push(packet);
        }

        remaining_bits = bits;
    } else {
        let number_of_subpackets = parse_value(&bits[1..12]);

        let mut bits = &bits[12..];

        for _ in 0..number_of_subpackets {
            let (packet, remaining) = parse_packet(bits);
            bits = remaining;
            subpackets.push(packet);
        }

        remaining_bits = bits;
    }

    let packet_type = PacketType::Operator {
        op: parse_packet_op(op_id),
        subpackets,
    };

    (packet_type, remaining_bits)
}

fn parse_packet(bits: &BitSlice<Msb0, u8>) -> (Packet, &BitSlice<Msb0, u8>) {
    let version = &bits[..3];
    let version = parse_value(version);

    let type_id = &bits[3..6];
    let type_id = parse_value(type_id);

    let (packet_type, remaining) = if type_id == 4 {
        parse_literal_packet(&bits[6..])
    } else {
        parse_operator_packet(&bits[6..], type_id)
    };

    let packet = Packet {
        version,
        packet_type,
    };
    (packet, remaining)
}

fn parse_hex_to_bitvec(input: &str) -> BitVec<Msb0, u8> {
    input
        .chars()
        .tuples()
        .map(|(a, b)| -> u8 {
            let first_digit = a.to_digit(16).unwrap();
            let second_digit = b.to_digit(16).unwrap();
            (first_digit << 4 | second_digit).try_into().unwrap()
        })
        .collect()
}

fn parse(input: &str) -> anyhow::Result<Packet> {
    let bitvec = parse_hex_to_bitvec(input.trim());
    let (packet, _) = parse_packet(&bitvec);
    Ok(packet)
}

fn sum_version_numbers(packet: &Packet) -> u64 {
    let mut s: u64 = packet.version as _;

    if let PacketType::Operator { subpackets, .. } = &packet.packet_type {
        s += subpackets.iter().map(sum_version_numbers).sum::<u64>();
    }

    s
}

fn eval_packet(packet: &Packet) -> u64 {
    use PacketOp::*;

    match &packet.packet_type {
        PacketType::Literal(l) => *l,
        PacketType::Operator {
            op: Sum,
            subpackets,
        } => subpackets.iter().map(eval_packet).sum(),
        PacketType::Operator {
            op: Product,
            subpackets,
        } => subpackets.iter().map(eval_packet).product(),
        PacketType::Operator {
            op: Max,
            subpackets,
        } => subpackets.iter().map(eval_packet).max().unwrap(),
        PacketType::Operator {
            op: Min,
            subpackets,
        } => subpackets.iter().map(eval_packet).min().unwrap(),
        PacketType::Operator {
            op: GreaterThan,
            subpackets,
        } => (eval_packet(&subpackets[0]) > eval_packet(&subpackets[1])) as u64,
        PacketType::Operator {
            op: LessThan,
            subpackets,
        } => (eval_packet(&subpackets[0]) < eval_packet(&subpackets[1])) as u64,
        PacketType::Operator {
            op: Equal,
            subpackets,
        } => (eval_packet(&subpackets[0]) == eval_packet(&subpackets[1])) as u64,
    }
}

pub fn part1(text: &str) -> anyhow::Result<u64> {
    let packet = parse(text)?;
    Ok(sum_version_numbers(&packet))
}

pub fn part2(text: &str) -> anyhow::Result<u64> {
    let packet = parse(text)?;
    Ok(eval_packet(&packet))
}
