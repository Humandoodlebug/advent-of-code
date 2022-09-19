use std::str::FromStr;

use util::PerfTimer;

extern crate util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let v: i32 = split[1]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        match split[0] {
            "forward" => Ok(Instruction::Forward(v)),
            "down" => Ok(Instruction::Down(v)),
            "up" => Ok(Instruction::Up(v)),
            x => Err(format!("Unrecognised instruction {:?}", x)),
        }
    }
}

fn input() -> Vec<Instruction> {
    util::get_day_input(2)
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let instructions = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let destination = instructions
            .iter()
            .map(|&instruction| match instruction {
                Instruction::Forward(x) => (x, 0),
                Instruction::Down(x) => (0, x),
                Instruction::Up(x) => (0, -x),
            })
            .reduce(|(x1, y1), (x2, y2)| (x1 + x2, y1 + y2))
            .unwrap();

        println!("Part 1: {}", destination.0 * destination.1);
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let destination_aim = instructions
            .iter()
            .fold((0, 0, 0), |(x, y, aim), &instruction| match instruction {
                Instruction::Forward(i) => (x + i, y + i * aim, aim),
                Instruction::Down(i) => (x, y, aim + i),
                Instruction::Up(i) => (x, y, aim - i),
            });

        println!("Part 2: {}", destination_aim.0 * destination_aim.1);
    }
}
