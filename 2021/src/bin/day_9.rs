use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

use itertools::Itertools;
use util::PerfTimer;

extern crate util;

fn input() -> Vec<Vec<i32>> {
    let raw = util::get_day_input(9);
    raw.lines()
        .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect()
}

fn main() {
    let map = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut sum = 0;
        for (x, line) in map.iter().enumerate() {
            for (y, &cell) in line.iter().enumerate() {
                let mut adjacent = vec![];
                if x > 0 {
                    adjacent.push((x - 1, y))
                }
                if x < map.len() - 1 {
                    adjacent.push((x + 1, y));
                }
                if y > 0 {
                    adjacent.push((x, y - 1));
                }
                if y < map[0].len() - 1 {
                    adjacent.push((x, y + 1));
                }

                if adjacent
                    .into_iter()
                    .map(|(i, j)| map[i][j])
                    .all(|v| v > cell)
                {
                    sum += cell + 1;
                }
            }
        }
        println!("Part 1: {}", sum);
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut points_to_basins = HashMap::new();
        let mut basins_to_points: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

        for (x, line) in map.iter().enumerate() {
            for (y, &cell) in line.iter().enumerate() {
                if cell == 9 {
                } else if x > 0 && map[x - 1][y] < 9 {
                    let &basin = points_to_basins.get(&(x - 1, y)).unwrap();
                    points_to_basins.insert((x, y), basin);
                    basins_to_points.get_mut(&basin).unwrap().insert((x, y));
                    if y > 0
                        && map[x][y - 1] < 9
                        && points_to_basins.get(&(x, y - 1)).unwrap() != &basin
                    {
                        let old_basin = points_to_basins.get(&(x, y - 1)).unwrap();
                        let points = basins_to_points.remove(old_basin).unwrap();
                        for p in &points {
                            points_to_basins.insert(*p, basin);
                        }
                        basins_to_points.get_mut(&basin).unwrap().extend(points);
                    }
                } else if y > 0 && map[x][y - 1] < 9 {
                    let &basin = points_to_basins.get(&(x, y - 1)).unwrap();
                    points_to_basins.insert((x, y), basin);
                    basins_to_points.get_mut(&basin).unwrap().insert((x, y));
                } else {
                    points_to_basins.insert((x, y), (x, y));
                    let mut b2p = HashSet::new();
                    b2p.insert((x, y));
                    basins_to_points.insert((x, y), b2p);
                }
            }
        }
        let basin_sizes = basins_to_points
            .into_values()
            .map(|s| s.len())
            .sorted()
            .rev()
            .take(3)
            .reduce(usize::mul)
            .unwrap();
        println!("Part 2: {}", basin_sizes);
    }
}
