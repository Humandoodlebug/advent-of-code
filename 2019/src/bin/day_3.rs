use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use Instruction::*;

enum Instruction {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

fn parse_instruction(s: &str) -> Instruction {
    let len: usize = s[1..].parse().unwrap();
    match s.chars().next().unwrap() {
        'U' => Up(len),
        'R' => Right(len),
        'D' => Down(len),
        'L' => Left(len),
        _ => panic!(),
    }
}

fn input() -> (Vec<Instruction>, Vec<Instruction>) {
    let raw = util::get_day_input(3);
    let mut lines = raw.lines();
    let raw_1 = lines.next().unwrap();
    let raw_2 = lines.next().unwrap();

    let wire_1 = raw_1.split(',').map(parse_instruction).collect();
    let wire_2 = raw_2.split(',').map(parse_instruction).collect();
    (wire_1, wire_2)
}

fn main() {
    let (wire_1, wire_2) = input();
    let mut wire_1_positions = HashSet::new();
    let mut wire_1_map = HashMap::new();

    let mut steps = 0;
    let mut position = (0, 0);
    for instruction in wire_1 {
        match instruction {
            Up(l) => {
                for _ in 0..l {
                    position.0 += 1;
                    steps += 1;
                    wire_1_positions.insert(position);
                    let _ = wire_1_map.entry(position).or_insert(steps);
                }
            }
            Right(l) => {
                for _ in 0..l {
                    position.1 += 1;
                    steps += 1;
                    wire_1_positions.insert(position);
                    let _ = wire_1_map.entry(position).or_insert(steps);
                }
            }
            Down(l) => {
                for _ in 0..l {
                    position.0 -= 1;
                    steps += 1;
                    wire_1_positions.insert(position);
                    let _ = wire_1_map.entry(position).or_insert(steps);
                }
            }
            Left(l) => {
                for _ in 0..l {
                    position.1 -= 1;
                    steps += 1;
                    wire_1_positions.insert(position);
                    let _ = wire_1_map.entry(position).or_insert(steps);
                }
            }
        }
    }

    let mut part_1 = usize::MAX;
    let mut part_2 = usize::MAX;
    let mut position = (0, 0);
    let mut len = 0;

    for instruction in wire_2 {
        match instruction {
            Up(l) => {
                for _ in 0..l {
                    position.0 += 1;
                    len += 1;
                    if wire_1_positions.contains(&position) {
                        let p = position.0 + position.1;
                        part_1 = min(part_1, p);
                        let p2 = wire_1_map.get(&position).unwrap();
                        part_2 = min(part_2, p2 + len);
                    }
                }
            }
            Right(l) => {
                for _ in 0..l {
                    position.1 += 1;
                    len += 1;
                    if wire_1_positions.contains(&position) {
                        let p = position.0 + position.1;
                        part_1 = min(part_1, p);
                        let p2 = wire_1_map.get(&position).unwrap();
                        part_2 = min(part_2, p2 + len);
                    }
                }
            }
            Down(l) => {
                for _ in 0..l {
                    position.0 -= 1;
                    len += 1;
                    if wire_1_positions.contains(&position) {
                        let p = position.0 + position.1;
                        part_1 = min(part_1, p);
                        let p2 = wire_1_map.get(&position).unwrap();
                        part_2 = min(part_2, p2 + len);
                    }
                }
            }
            Left(l) => {
                for _ in 0..l {
                    position.1 -= 1;
                    len += 1;
                    if wire_1_positions.contains(&position) {
                        let p = position.0 + position.1;
                        part_1 = min(part_1, p);
                        let p2 = wire_1_map.get(&position).unwrap();
                        part_2 = min(part_2, p2 + len);
                    }
                }
            }
        }
    }
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
