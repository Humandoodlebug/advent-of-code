use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::PerfTimer;

extern crate util;

type Point = (i32, i32, i32);

type Scanner = Vec<Point>;

fn input() -> Vec<Scanner> {
    let raw = util::get_day_input(19);
    let mut scanners = Vec::new();
    let mut lines = raw.lines();
    loop {
        let _scanner_line = lines.next().unwrap();
        let mut scanner = Vec::new();
        loop {
            let line = lines.next();
            if line.is_none() {
                scanners.push(scanner);
                return scanners;
            }
            if line.unwrap().is_empty() {
                break;
            }
            scanner.push(
                line.unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        }
        scanners.push(scanner);
    }
}

fn mul(point_a: Point, point_b: Point) -> Point {
    (
        point_a.0 * point_b.0,
        point_a.1 * point_b.1,
        point_a.2 * point_b.2,
    )
}

fn dot(point_a: Point, point_b: Point) -> i32 {
    let p = mul(point_a, point_b);
    p.0 + p.1 + p.2
}

fn mat_dot(point: Point, mat: [Point; 3]) -> Point {
    (dot(point, mat[0]), dot(point, mat[1]), dot(point, mat[2]))
}

fn rotate_all_90(point: Point) -> Vec<Point> {
    const MAT_X: [(i32, i32, i32); 3] = [(1, 0, 0), (0, 0, -1), (0, 1, 0)];
    const MAT_Y: [(i32, i32, i32); 3] = [(0, 0, 1), (0, 1, 0), (-1, 0, 0)];
    const MAT_Z: [(i32, i32, i32); 3] = [(0, -1, 0), (1, 0, 0), (0, 0, 1)];
    let mut points = vec![point];
    let mut temp = point;
    for _ in 0..3 {
        temp = mat_dot(temp, MAT_X);
        points.push(temp);
    }
    for p in points.clone() {
        let mut temp = p;
        temp = mat_dot(temp, MAT_Y);
        points.push(temp);
        temp = mat_dot(mat_dot(temp, MAT_Y), MAT_Y);
        points.push(temp);
    }

    for p in points.clone() {
        let mut temp = p;
        for _ in 0..3 {
            temp = mat_dot(temp, MAT_Z);
            points.push(temp);
        }
    }

    points
}

fn possible_scanner_positions_from_common_beacon(
    beacon_from_origin: Point,
    beacon_from_scanner: Point,
) -> Vec<Point> {
    let (x_o, y_o, z_o) = beacon_from_origin;

    rotate_all_90(beacon_from_scanner)
        .into_iter()
        .map(|(x, y, z)| (x_o - x, y_o - y, z_o - z))
        .collect()
}

fn relative_beacon_to_origin(
    scanner_location: Point,
    rotation_index: usize,
    beacon_location: Point,
) -> Point {
    let beacon_location = rotate_all_90(beacon_location)[rotation_index];
    (
        scanner_location.0 + beacon_location.0,
        scanner_location.1 + beacon_location.1,
        scanner_location.2 + beacon_location.2,
    )
}

fn locate_scanner(
    beacons_from_origin: &[Point],
    beacons_from_scanner: &[Point],
) -> Option<(usize, Point)> {
    let mut beacon_locations = HashMap::new();
    for &b_o in beacons_from_origin {
        for &b_s in beacons_from_scanner {
            let possible_locations = possible_scanner_positions_from_common_beacon(b_o, b_s);
            for (i, p) in possible_locations.into_iter().enumerate() {
                if let Some(v) = beacon_locations.get_mut(&(i, p)) {
                    *v += 1;
                } else {
                    beacon_locations.insert((i, p), 1);
                }
            }
        }
    }
    let best = beacon_locations.into_iter().max_by_key(|x| x.1).unwrap();
    if best.1 >= 12 {
        Some(best.0)
    } else {
        None
    }
}

fn main() {
    let inp = input();
    // dbg!(inp);

    let part_1_timer = PerfTimer::new("Part 1");
    let part_2_timer = PerfTimer::new("Part 2");
    let mut origin = inp[0].clone();
    let mut remaining: HashMap<usize, Vec<(i32, i32, i32)>> =
        inp[1..].iter().cloned().enumerate().collect();
    let mut scanner_locations: HashMap<usize, Point> = HashMap::new();
    let mut locations: HashSet<Point> = HashSet::new();
    for p in origin.clone() {
        locations.insert(p);
    }
    while !remaining.is_empty() {
        let mut left = HashMap::new();
        for (k, v) in remaining {
            if let Some((rotation_index, scanner_loc)) = locate_scanner(&origin, &v) {
                scanner_locations.insert(k, scanner_loc);
                for beacon in v {
                    let beacon_loc = relative_beacon_to_origin(scanner_loc, rotation_index, beacon);
                    if locations.insert(beacon_loc) {
                        origin.push(beacon_loc);
                    }
                }
            } else {
                left.insert(k, v);
            }
        }
        remaining = left;
    }
    println!("Part 1: {}", locations.len());
    drop(part_1_timer);

    let mut part2 = 0;
    for &(x_a, y_a, z_a) in scanner_locations.values() {
        for &(x_b, y_b, z_b) in scanner_locations.values() {
            let distance = x_a.abs_diff(x_b) + y_a.abs_diff(y_b) + z_a.abs_diff(z_b);
            part2 = std::cmp::max(part2, distance);
        }
    }
    println!("Part 2: {}", part2);
    drop(part_2_timer);
}
