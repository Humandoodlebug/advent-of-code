use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use regex::Regex;
use util::PerfTimer;

type Point = (i128, i128);

#[derive(Clone, Copy, Debug)]
struct Sensor {
    pos: Point,
    beacon_pos: Point,
}

struct Diamond {
    left: Point,
    right: Point,
    top: Point,
    bottom: Point,
}

impl Diamond {
    // fn area(&self) -> i128 {
    //     self.top.
    // }
    // fn try_new(left: Point, right: Point) -> Option<Diamond> {

    // }

    // fn left(&self) -> Point {
    //     self.left
    // }

    // fn right(&self) -> Point {
    //     self.right
    // }

    // fn top(&self) -> Point {
    //     let from_left_double = (self.right.0 - self.left.0) + (self.left.1 - self.right.1);
    //     assert!(from_left_double % 2 == 0);
    //     let from_left = from_left_double / 2;
    //     (self.left.0 + from_left, self.left.1 - from_left)
    // }

    // fn bottom(&self) -> Point {
    //     let from_left_double = (self.right.0 - self.left.0) - (self.left.1 - self.right.1);
    //     assert!(from_left_double % 2 == 0);
    //     let from_left = from_left_double / 2;
    //     (self.left.0 + from_left, self.left.1 + from_left)
    // }
}

#[derive(Default)]
struct RangeManager {
    ranges: HashSet<(i128, i128)>,
}

impl RangeManager {
    fn add_range(&mut self, range: (i128, i128)) {
        let lower = self
            .ranges
            .iter()
            .copied()
            .find(|r| r.1 + 1 >= range.0 && r.0 <= range.0);
        let upper = self
            .ranges
            .iter()
            .copied()
            .find(|r| r.0 - 1 <= range.1 && r.1 >= range.1);

        // Early return if an existing range entirely contains us
        if lower.is_some() && lower == upper {
            return;
        }

        let left = if let Some(lower) = lower {
            self.ranges.remove(&lower);
            lower.0
        } else {
            range.0
        };

        let right = if let Some(upper) = upper {
            self.ranges.remove(&upper);
            upper.1
        } else {
            range.1
        };

        // Remove sub-ranges
        let to_remove: Vec<Point> = self
            .ranges
            .iter()
            .copied()
            .filter(|r| r.0 >= left && r.1 <= right)
            .collect();

        for p in to_remove {
            self.ranges.remove(&p);
        }

        self.ranges.insert((left, right));
    }

    fn sum_ranges(&self) -> i128 {
        self.ranges.iter().map(|(l, r)| r - l + 1).sum()
    }
}

fn input() -> Vec<Sensor> {
    let re =
        Regex::new(r#"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"#)
            .unwrap();
    util::get_day_input(15)
        .trim()
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let pos = (
                captures.name("sensor_x").unwrap().as_str().parse().unwrap(),
                captures.name("sensor_y").unwrap().as_str().parse().unwrap(),
            );
            let beacon_pos = (
                captures.name("beacon_x").unwrap().as_str().parse().unwrap(),
                captures.name("beacon_y").unwrap().as_str().parse().unwrap(),
            );
            Sensor { pos, beacon_pos }
        })
        .collect()
}

fn manhattan((x_a, y_a): Point, (x_b, y_b): Point) -> u128 {
    y_b.abs_diff(y_a) + x_b.abs_diff(x_a)
}

fn main() {
    let sensors = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let search_line = 2000000;

        let mut line_ranges = RangeManager::default();
        let mut beacons_on_line = HashSet::new();

        for sensor in &sensors {
            if sensor.beacon_pos.1 == search_line {
                beacons_on_line.insert(sensor.beacon_pos);
            }
            let radius = manhattan(sensor.pos, sensor.beacon_pos);
            let line_radius = radius as i128 - sensor.pos.1.abs_diff(search_line) as i128;
            if line_radius >= 0 {
                let left = sensor.pos.0 - line_radius;
                let right = sensor.pos.0 + line_radius;
                line_ranges.add_range((left, right));
            }
        }

        let part_1 = line_ranges.sum_ranges() - beacons_on_line.len() as i128;
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let range_min = 0;
        let range_max = 4_000_000;

        for search_line in range_min..=range_max {
            let mut line_ranges = RangeManager::default();
            let mut beacons_on_line = HashSet::new();

            for sensor in &sensors {
                if sensor.beacon_pos.1 == search_line {
                    beacons_on_line.insert(sensor.beacon_pos);
                }
                let radius = manhattan(sensor.pos, sensor.beacon_pos);
                let line_radius = radius as i128 - sensor.pos.1.abs_diff(search_line) as i128;
                if line_radius >= 0 {
                    let left = (sensor.pos.0 - line_radius).min(range_max).max(range_min);
                    let right = (sensor.pos.0 + line_radius).min(range_max).max(range_min);
                    line_ranges.add_range((left, right));
                }
            }

            if line_ranges.sum_ranges() < range_max - range_min + 1 {
                println!("Line: {search_line}");
                let line_ranges = line_ranges.ranges.iter().collect_vec();
                let x_coord = if line_ranges.len() == 2 {
                    let over = line_ranges.iter().max_by_key(|r| r.0).unwrap().0;
                    let under = line_ranges.iter().min_by_key(|r| r.1).unwrap().1;
                    assert_eq!(over - under, 2);
                    over - 1
                } else if line_ranges.len() == 1 {
                    let line_range = line_ranges.first().unwrap();
                    if line_range.0 == range_min + 1 {
                        range_min
                    } else if line_range.1 == range_max - 1 {
                        range_max
                    } else {
                        panic!("Single range is not almost-total: {line_range:?}");
                    }
                } else {
                    panic!("Expected 1 or 2 line ranges, but got {}", line_ranges.len())
                };
                let part_2 = x_coord * 4_000_000 + search_line;
                println!("Part 2: {part_2}");
                break;
            }
        }
    }
}
