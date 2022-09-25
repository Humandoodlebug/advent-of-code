#![feature(array_windows)]

use std::cmp::Ordering;

use util::{get_day_input, PerfTimer};

fn input() -> (u64, u64) {
    let raw = get_day_input(4);
    let (left, right) = raw.trim().split_once('-').unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

enum DoubleState {
    NotFound,
    FoundPair,
    CompletedPair,
    FoundMoreThanPair,
    CompletedMoreThanPair,
}

fn main() {
    let (lower, upper) = input();

    let _timer = PerfTimer::new("Both parts");
    let mut part_1 = 0;
    let mut part_2 = 0;
    for i in lower..=upper {
        let s = i.to_string();
        let mut prev = None;
        let mut double = DoubleState::NotFound;
        let mut increasing = true;
        for c in s.chars() {
            if let Some(prev) = prev {
                match c.cmp(&prev) {
                    Ordering::Less => {
                        increasing = false;
                        break;
                    }
                    Ordering::Equal => {
                        double = match double {
                            DoubleState::NotFound => DoubleState::FoundPair,
                            DoubleState::FoundPair => DoubleState::FoundMoreThanPair,
                            DoubleState::CompletedPair => DoubleState::CompletedPair,
                            DoubleState::FoundMoreThanPair => DoubleState::FoundMoreThanPair,
                            DoubleState::CompletedMoreThanPair => DoubleState::FoundPair,
                        }
                    }
                    Ordering::Greater => {
                        double = match double {
                            DoubleState::NotFound => DoubleState::NotFound,
                            DoubleState::FoundPair => DoubleState::CompletedPair,
                            DoubleState::CompletedPair => DoubleState::CompletedPair,
                            DoubleState::FoundMoreThanPair => DoubleState::CompletedMoreThanPair,
                            DoubleState::CompletedMoreThanPair => {
                                DoubleState::CompletedMoreThanPair
                            }
                        }
                    }
                }
            }
            prev = Some(c);
        }
        if increasing {
            match double {
                DoubleState::FoundPair | DoubleState::CompletedPair => {
                    part_1 += 1;
                    part_2 += 1;
                }
                DoubleState::FoundMoreThanPair | DoubleState::CompletedMoreThanPair => {
                    part_1 += 1;
                }
                DoubleState::NotFound => {}
            }
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
