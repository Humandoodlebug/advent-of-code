use std::collections::HashSet;

use itertools::Itertools;
use util::PerfTimer;

type Point = (usize, usize);

fn input() -> Vec<Vec<Point>> {
    util::get_day_input(14)
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (left, right) = point.split_once(',').unwrap();
                    (left.parse().unwrap(), right.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn build_rock_positions(rock_paths: &Vec<Vec<Point>>) -> HashSet<Point> {
    let mut rock_positions = HashSet::new();
    for rock_path in rock_paths {
        for ((x_1, y_1), (x_2, y_2)) in rock_path.iter().copied().tuple_windows() {
            if x_1 == x_2 {
                for y in y_1.min(y_2)..=y_1.max(y_2) {
                    rock_positions.insert((x_1, y));
                }
            } else if y_1 == y_2 {
                for x in x_1.min(x_2)..=x_1.max(x_2) {
                    rock_positions.insert((x, y_1));
                }
            } else {
                panic!();
            }
        }
    }
    rock_positions
}

fn find_floor(rock_positions: &HashSet<Point>) -> usize {
    rock_positions.iter().map(|p| p.1).max().unwrap()
}

fn step_sand(
    rock_positions: &HashSet<Point>,
    sand_positions: &HashSet<Point>,
    (x, y): Point,
) -> Option<Point> {
    if !rock_positions.contains(&(x, y + 1)) && !sand_positions.contains(&(x, y + 1)) {
        Some((x, y + 1))
    } else if !rock_positions.contains(&(x - 1, y + 1)) && !sand_positions.contains(&(x - 1, y + 1))
    {
        Some((x - 1, y + 1))
    } else if !rock_positions.contains(&(x + 1, y + 1)) && !sand_positions.contains(&(x + 1, y + 1))
    {
        Some((x + 1, y + 1))
    } else {
        None
    }
}

fn place_sand_1(
    rock_positions: &HashSet<Point>,
    sand_positions: &HashSet<Point>,
    floor_level: usize,
) -> Option<Point> {
    let mut point = (500, 0);
    loop {
        if let Some((x, y)) = step_sand(rock_positions, sand_positions, point) {
            if y < floor_level {
                point = (x, y);
            } else {
                return None;
            }
        } else {
            return Some(point);
        }
    }
}

fn place_sand_2(
    rock_positions: &HashSet<Point>,
    sand_positions: &HashSet<Point>,
    floor_level: usize,
) -> Point {
    let mut point = (500, 0);
    loop {
        if let Some((x, y)) = step_sand(rock_positions, sand_positions, point) {
            if y < floor_level {
                point = (x, y);
            } else {
                return point;
            }
        } else {
            return point;
        }
    }
}

fn main() {
    let rock_paths = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let rock_positions = build_rock_positions(&rock_paths);
        let max_y = find_floor(&rock_positions);
        let mut sand_positions = HashSet::new();

        while let Some(point) = place_sand_1(&rock_positions, &sand_positions, max_y) {
            sand_positions.insert(point);
        }

        let part_1 = sand_positions.len();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let rock_positions = build_rock_positions(&rock_paths);
        let floor_level = find_floor(&rock_positions) + 2;
        let mut sand_positions = HashSet::new();

        loop {
            let point = place_sand_2(&rock_positions, &sand_positions, floor_level);
            sand_positions.insert(point);
            if point == (500, 0) {
                break;
            }
        }

        let part_2 = sand_positions.len();
        println!("Part 2: {part_2}");
    }
}
