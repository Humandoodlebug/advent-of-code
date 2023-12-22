use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::PerfTimer;

type Point = (i64, i64, i64);
type Brick = (Point, Point);

fn input() -> Vec<Brick> {
    util::get_day_input(22)
        .lines()
        .map(|line| {
            line.split('~')
                .map(|p| {
                    p.split(',')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn settle(bricks: &[Brick]) -> Vec<Brick> {
    let mut settled_cubes: HashSet<Point> = HashSet::new();
    let mut settled_bricks: Vec<(Point, Point)> = Vec::new();

    for (p1, p2) in bricks.iter().sorted_by_key(|&(p1, p2)| p1.2.min(p2.2)) {
        let (mut z1, mut z2) = (p1.2, p2.2);
        'a: while z1.min(z2) > 1 {
            for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    if settled_cubes.contains(&(x, y, z1.min(z2) - 1)) {
                        break 'a;
                    }
                }
            }
            z1 -= 1;
            z2 -= 1;
        }
        settled_bricks.push(((p1.0, p1.1, z1), (p2.0, p2.1, z2)));
        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                for z in z1.min(z2)..=z1.max(z2) {
                    settled_cubes.insert((x, y, z));
                }
            }
        }
    }

    settled_bricks
}

fn build_resting_map(bricks: &[Brick]) -> HashMap<Brick, Vec<Brick>> {
    let mut cubes = HashMap::new();
    for &brick @ (p1, p2) in bricks {
        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                for z in p1.2.min(p2.2)..=p1.2.max(p2.2) {
                    cubes.insert((x, y, z), brick);
                }
            }
        }
    }
    let mut resting_map = HashMap::new();
    for &brick @ (p1, p2) in bricks {
        let mut resting_on = HashSet::new();
        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                let z = p1.2.min(p2.2) - 1;
                if let Some(&b) = cubes.get(&(x, y, z)) {
                    resting_on.insert(b);
                }
            }
        }
        resting_map.insert(brick, resting_on.into_iter().collect());
    }
    resting_map
}

fn main() {
    let bricks = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let settled_bricks = settle(&bricks);
        let resting_map = build_resting_map(&settled_bricks);
        let cannot_disintegrate = resting_map
            .values()
            .filter(|bs| bs.len() == 1)
            .map(|bs| bs[0])
            .unique()
            .count();

        let part_1 = bricks.len() - cannot_disintegrate;

        println!("Part 1: {}", part_1);
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let settled_bricks = settle(&bricks);
        let resting_map = build_resting_map(&settled_bricks);

        let part_2: u64 = settled_bricks
            .iter()
            .map(|&disintegrated_brick| {
                let mut would_fall = HashSet::new();
                would_fall.insert(disintegrated_brick);
                for &brick @ (p1, p2) in settled_bricks
                    .iter()
                    .sorted_by_key(|(p1, p2)| p1.2.min(p2.2))
                {
                    if p1.2.min(p2.2) > 1
                        && resting_map[&brick].iter().all(|b| would_fall.contains(b))
                    {
                        would_fall.insert(brick);
                    }
                }
                would_fall.len() as u64 - 1
            })
            .sum();

        println!("Part 2: {part_2}");
    }
}
