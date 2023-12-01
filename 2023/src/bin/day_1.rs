use itertools::Itertools;
use util::PerfTimer;

extern crate util;

fn input() -> Vec<String> {
    util::get_day_input(1).lines().map(String::from).collect()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let lines = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: u32 = lines
            .iter()
            .map(|s| {
                let digit1 = s.chars().find(|c| c.is_ascii_digit()).unwrap();
                let digit2 = s.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
                [digit1, digit2]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .sum();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let part_2: u32 = lines
            .iter()
            .map(|s| {
                let (a_digit1_pos, a_digit1) =
                    s.chars().find_position(|c| c.is_ascii_digit()).unwrap();
                let (a_digit2_pos, a_digit2) = s
                    .chars()
                    .enumerate()
                    .collect_vec()
                    .into_iter()
                    .rev()
                    .find(|(_, c)| c.is_ascii_digit())
                    .unwrap();

                let b_digit1 = DIGITS
                    .iter()
                    .zip('1'..)
                    .filter_map(|(&d, i)| Some((s.find(d)?, i)))
                    .min_by_key(|&(p, _)| p);
                let b_digit2 = DIGITS
                    .iter()
                    .zip('1'..)
                    .filter_map(|(&d, i)| Some((s.rfind(d)?, i)))
                    .max_by_key(|&(p, _)| p);

                let digit1 = if b_digit1.is_some_and(|(p, _)| p < a_digit1_pos) {
                    b_digit1.unwrap().1
                } else {
                    a_digit1
                };
                let digit2 = if b_digit2.is_some_and(|(p, _)| p > a_digit2_pos) {
                    b_digit2.unwrap().1
                } else {
                    a_digit2
                };

                [digit1, digit2]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .sum();

        println!("Part 2: {part_2}");
    }
}
