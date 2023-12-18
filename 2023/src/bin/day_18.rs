#![feature(never_type)]
#![feature(array_windows)]

use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InstructionLine {
    direction: Direction,
    distance: u64,
    colour: u32,
}

fn input() -> Vec<InstructionLine> {
    util::get_day_input(18)
        .lines()
        .map(|line| {
            let (direction, distance, colour) = line.split(' ').collect_tuple().unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!(),
            };
            let distance = distance.parse().unwrap();
            let colour = u32::from_str_radix(colour.trim_matches(&['(', ')', '#']), 16).unwrap();
            InstructionLine {
                direction,
                distance,
                colour,
            }
        })
        .collect()
}

fn find_vertices(instructions: &[Instruction]) -> Vec<(i64, i64)> {
    let mut vertexes = Vec::with_capacity(instructions.len());
    let mut position = (0, 0);
    vertexes.push(position);
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                position.0 -= instruction.distance as i64;
            }
            Direction::Right => {
                position.1 += instruction.distance as i64;
            }
            Direction::Down => {
                position.0 += instruction.distance as i64;
            }
            Direction::Left => {
                position.1 -= instruction.distance as i64;
            }
        }
        vertexes.push(position);
    }
    vertexes
}

fn find_area(vertexes: &[(i64, i64)]) -> u64 {
    let base = vertexes
        .iter()
        .copied()
        .tuple_windows()
        .map(|(v1, v2)| v1.0 * v2.1 - v1.1 * v2.0)
        .sum::<i64>()
        .unsigned_abs()
        / 2;

    let edge_len = vertexes
        .iter()
        .copied()
        .tuple_windows()
        .map(|(v1, v2)| v1.0.abs_diff(v2.0) + v1.1.abs_diff(v2.1))
        .sum::<u64>()
        + 1;

    base + edge_len / 2 + 1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    distance: u64,
}

fn main() {
    let instructions = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let instructions = instructions
            .iter()
            .map(|instruction| Instruction {
                direction: instruction.direction,
                distance: instruction.distance,
            })
            .collect_vec();
        let vertexes = find_vertices(&instructions);
        let part_1 = find_area(&vertexes);
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let instructions = instructions
            .iter()
            .map(|instruction| Instruction {
                distance: (instruction.colour / 16) as u64,
                direction: match instruction.colour % 16 {
                    0 => Direction::Right,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    3 => Direction::Up,
                    _ => panic!(),
                },
            })
            .collect_vec();
        let vertexes = find_vertices(&instructions);
        let part_2 = find_area(&vertexes);
        println!("Part 2: {part_2}");
    }
}
