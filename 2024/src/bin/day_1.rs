use std::collections::HashMap;

use util::PerfTimer;

extern crate util;

fn input() -> Vec<(u64, u64)> {
    let input = util::get_day_input(1);
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

fn main() {
    let input = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut list_a: Vec<u64> = input.iter().map(|(a, _)| *a).collect();
        let mut list_b: Vec<u64> = input.iter().map(|(_, b)| *b).collect();
        list_a.sort();
        list_b.sort();

        let part_1: u64 = list_a
            .iter()
            .zip(list_b.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let list_a: Vec<u64> = input.iter().map(|(a, _)| *a).collect();
        let list_b: Vec<u64> = input.iter().map(|(_, b)| *b).collect();

        let mut b_counts = HashMap::new();
        for &x in list_b.iter() {
            *b_counts.entry(x).or_insert(0) += 1;
        }

        let part_2: u64 = list_a
            .iter()
            .map(|&a| a * b_counts.get(&a).unwrap_or(&0))
            .sum();
        println!("Part 2: {part_2}");
    }
}
