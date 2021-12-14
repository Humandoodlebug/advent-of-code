#![feature(array_windows)]
#![feature(iter_intersperse)]
#![feature(slice_group_by)]
use std::collections::HashMap;

use itertools::Itertools;

extern crate util;

fn input() -> (Vec<char>, HashMap<(char, char), char>) {
    let raw = util::get_day_input(14);
    let mut lines = raw.lines();
    let template = lines.next().unwrap().chars().collect();
    lines.next();
    let rules = lines
        .map(|l| {
            let mut chars = l.chars();
            let a = chars.next().unwrap();
            let b = chars.next().unwrap();
            let c = chars.nth(4).unwrap();
            ((a, b), c)
        })
        .collect();
    (template, rules)
}

fn main() {
    let (template, rules) = input();

    let polymer_counts = template.array_windows().map(|&[a, b]| (a, b)).counts();
    let mut polymer_counts: HashMap<(char, char), u128> = polymer_counts
        .into_iter()
        .map(|(k, v)| (k, v as u128))
        .collect();
    let mut counts: HashMap<char, u128> = template
        .iter()
        .counts()
        .into_iter()
        .map(|(&k, v)| (k, v as u128))
        .collect();

    for step in 1..=40 {
        let mut new_polymers = HashMap::new();
        for (&(a, b), &count) in polymer_counts.iter() {
            let &c = rules.get(&(a, b)).unwrap();
            if let Some(x) = new_polymers.get_mut(&(a, c)) {
                *x += count;
            } else {
                new_polymers.insert((a, c), count);
            }
            if let Some(x) = new_polymers.get_mut(&(c, b)) {
                *x += count;
            } else {
                new_polymers.insert((c, b), count);
            }
            if let Some(x) = counts.get_mut(&c) {
                *x += count;
            } else {
                counts.insert(c, count);
            }
        }
        polymer_counts = new_polymers;
        if step == 10 {
            let count_min = counts.values().min().unwrap();
            let count_max = counts.values().max().unwrap();
            let part1 = count_max - count_min;
            println!("Part 1: {}", part1);
        }
    }

    let count_min = counts.values().min().unwrap();
    let count_max = counts.values().max().unwrap();
    let part2 = count_max - count_min;
    println!("Part 2: {}", part2);
}
