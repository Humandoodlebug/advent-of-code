use std::collections::HashSet;

use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<(Vec<char>, Vec<char>)> {
    util::get_day_input(3)
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            (l.chars().collect(), r.chars().collect())
        })
        .collect()
}

fn priority(c: char) -> i64 {
    if ('a'..='z').contains(&c) {
        (c as i64) - ('a' as i64) + 1
    } else if ('A'..='Z').contains(&c) {
        (c as i64) - ('A' as i64) + 27
    } else {
        panic!("Unhandled letter {c:?}")
    }
}

fn main() {
    let rucksacks = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: i64 = rucksacks
            .iter()
            .map(|(left, right)| {
                let left: HashSet<char> = HashSet::from_iter(left.iter().copied());
                let right: HashSet<char> = HashSet::from_iter(right.iter().copied());
                left.intersection(&right).copied().map(priority).sum::<i64>()
            })
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2: i64 = rucksacks
            .iter()
            .map(|(left, right)| {
                HashSet::from_iter(left.iter().copied().chain(right.iter().copied()))
            })
            .chunks(3)
            .into_iter()
            .map(|rs| {
                rs.reduce(|s: HashSet<char>, x| HashSet::from_iter(s.intersection(&x).copied()))
                    .unwrap()
                    .into_iter()
                    .map(priority)
                    .sum::<i64>()
            })
            .sum();
        println!("Part 2: {part_2}");
    }
}
