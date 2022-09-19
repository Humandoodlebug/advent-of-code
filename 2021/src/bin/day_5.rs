#![allow(clippy::type_complexity)]

use std::cmp::{max, min};
use std::collections::HashMap;

use util::PerfTimer;

extern crate util;

fn input() -> Vec<((usize, usize), (usize, usize))> {
    util::get_day_input(5)
        .lines()
        .map(|l| {
            let mut parts = l.split(" -> ");
            let mut left = parts.next().unwrap().split(',');
            let mut right = parts.next().unwrap().split(',');

            let x1 = left.next().unwrap().parse().unwrap();
            let y1 = left.next().unwrap().parse().unwrap();
            let x2 = right.next().unwrap().parse().unwrap();
            let y2 = right.next().unwrap().parse().unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect()
}

fn run(inp: &[((usize, usize), (usize, usize))], part2: bool) -> usize {
    let mut map: HashMap<(usize, usize), i32> = HashMap::new();
    for &((x1, y1), (x2, y2)) in inp {
        let x_min = min(x1, x2);
        let x_max = max(x1, x2);
        let y_min = min(y1, y2);
        let y_max = max(y1, y2);

        if x1 != x2 && y1 != y2 {
            if part2 && x_max - x_min == y_max - y_min {
                let len = x_max - x_min;
                for i in 0..=len {
                    let (x, y) = if x_min == x1 && y_min == y1 || x_min == x2 && y_min == y2 {
                        (x_min + i, y_min + i)
                    } else {
                        (x_min + i, y_max - i)
                    };

                    if let Some(v) = map.get_mut(&(x, y)) {
                        *v += 1;
                    } else {
                        map.insert((x, y), 1);
                    }
                }
            }
        } else {
            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    if let Some(v) = map.get_mut(&(x, y)) {
                        *v += 1;
                    } else {
                        map.insert((x, y), 1);
                    }
                }
            }
        }
    }
    map.values().filter(|&&x| x >= 2).count()
}

fn main() {
    let inp = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part1 = run(&inp, false);
        println!("Part 1: {part1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let part2 = run(&inp, true);
        println!("Part 2: {part2}");
    }
}
