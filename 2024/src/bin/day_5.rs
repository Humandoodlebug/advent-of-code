use std::{borrow::BorrowMut, collections::HashSet};

use util::PerfTimer;

fn input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let raw = util::get_day_input(5);
    let mut lines = raw.lines();

    let rules = lines
        .borrow_mut()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let (l, r) = line.split_once('|').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let updates = lines
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn main() {
    let (rules, updates) = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let rules: HashSet<(u32, u32)> = rules.iter().copied().collect();
        let part_1: u32 = updates
            .iter()
            .filter(|update| {
                for i in 0..update.len() - 1 {
                    for j in i + 1..update.len() {
                        if rules.contains(&(update[j], update[i])) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|update| {
                assert!(
                    update.len() % 2 == 1,
                    "update lengths must be odd for there to be a middle element"
                );
                update[update.len() / 2]
            })
            .sum();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let rules: HashSet<(u32, u32)> = rules.iter().copied().collect();
        let part_2: u32 = updates
            .iter()
            .cloned()
            .filter_map(|mut update| {
                let mut changed = false;
                'outer: loop {
                    for i in 0..update.len() - 1 {
                        for j in i + 1..update.len() {
                            if rules.contains(&(update[j], update[i])) {
                                update.swap(i, j);
                                changed = true;
                                continue 'outer;
                            }
                        }
                    }

                    break;
                }

                if changed {
                    Some(update[update.len() / 2])
                } else {
                    None
                }
            })
            .sum();

        println!("Part 2: {part_2}");
    }
}
