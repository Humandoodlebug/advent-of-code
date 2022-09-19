use util::PerfTimer;
use Contents::*;
use OperatorType::*;

extern crate util;

fn input() -> Vec<i32> {
    util::get_day_input(16)
        .lines()
        .next()
        .unwrap()
        .chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => panic!(),
        })
        .collect()
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl OperatorType {
    fn parse(value: i128) -> Self {
        match value {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Contents {
    Literal(i128),
    Operator(OperatorType, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: i128,
    contents: Contents,
}

fn convert_to_num(num: &[i32]) -> i128 {
    let mut res = 0i128;
    for &digit in num {
        res = res * 2 + digit as i128
    }
    res
}

fn parse_num(inp: &[i32], len: usize) -> (&[i32], i128) {
    (&inp[len..], convert_to_num(&inp[..len]))
}

fn parse_variable_num(mut inp: &[i32]) -> (&[i32], i128) {
    let mut num = vec![];
    loop {
        let cont_bit = inp[0];
        for &x in &inp[1..=4] {
            num.push(x);
        }
        inp = &inp[5..];
        if cont_bit == 0 {
            break;
        }
    }
    (inp, convert_to_num(&num))
}

fn parse_packet(inp: &[i32]) -> (&[i32], Packet) {
    let (inp, version) = parse_num(inp, 3);
    let (inp, type_id) = parse_num(inp, 3);
    match type_id {
        4 => {
            let (inp, lit) = parse_variable_num(inp);
            (
                inp,
                Packet {
                    version,
                    contents: Literal(lit),
                },
            )
        }
        type_id => match inp[0] {
            0 => {
                let mut packets = vec![];
                let (inp, length_in_bits) = parse_num(&inp[1..], 15);
                let (mut remaining, inp) = inp.split_at(length_in_bits as usize);
                while !remaining.is_empty() {
                    let (rest, packet) = parse_packet(remaining);
                    packets.push(packet);
                    remaining = rest;
                }
                (
                    inp,
                    Packet {
                        version,
                        contents: Operator(OperatorType::parse(type_id), packets),
                    },
                )
            }
            1 => {
                let mut packets = vec![];
                let (mut remaining, num_packets) = parse_num(&inp[1..], 11);
                for _ in 0..num_packets {
                    let (rest, packet) = parse_packet(remaining);
                    packets.push(packet);
                    remaining = rest;
                }
                (
                    remaining,
                    Packet {
                        version,
                        contents: Operator(OperatorType::parse(type_id), packets),
                    },
                )
            }
            _ => panic!(),
        },
    }
}

fn sum_versions(packet: &Packet) -> i128 {
    match &packet.contents {
        Literal(_) => packet.version,
        Operator(_, packets) => packet.version + packets.iter().map(sum_versions).sum::<i128>(),
    }
}

fn eval_packet(packet: &Packet) -> i128 {
    match &packet.contents {
        Literal(x) => *x,
        Operator(op, packets) => {
            let mut values = packets.iter().map(eval_packet);
            match op {
                Sum => values.sum(),
                Product => values.product(),
                Minimum => values.min().unwrap(),
                Maximum => values.max().unwrap(),
                GreaterThan | LessThan | EqualTo => {
                    let left = values.next().unwrap();
                    let right = values.next().unwrap();
                    assert_eq!(values.next(), None);
                    let actual_op = match op {
                        GreaterThan => i128::gt,
                        LessThan => i128::lt,
                        EqualTo => i128::eq,
                        _ => panic!(),
                    };
                    i128::from(actual_op(&left, &right))
                }
            }
        }
    }
}

fn main() {
    let inp = input();

    let part_1_timer = PerfTimer::new("Part 1");
    let part_2_timer = PerfTimer::new("Part 2");

    let (_left, root_packet) = parse_packet(&inp);
    let part1 = sum_versions(&root_packet);
    println!("Part 1: {}", part1);
    drop(part_1_timer);

    let part2 = eval_packet(&root_packet);
    println!("Part 2: {}", part2);
    drop(part_2_timer);
}
