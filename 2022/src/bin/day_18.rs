use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::PerfTimer;

type Point = (i64, i64, i64);

fn input() -> Vec<Point> {
    let raw = util::get_day_input(18);
    raw.lines()
        .map(|l| {
            l.splitn(3, ',')
                .map(|s| s.parse().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect()
}

fn add_point((a_x, a_y, a_z): Point, (b_x, b_y, b_z): Point) -> Point {
    (a_x + b_x, a_y + b_y, a_z + b_z)
}

fn get_adjacent_positions(point: Point) -> [Point; 6] {
    let mut positions = [
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];
    for p in &mut positions {
        *p = add_point(*p, point)
    }
    positions
}

fn main() {
    let positions = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let positions: HashSet<Point> = HashSet::from_iter(positions.clone());
        let part_1: usize = positions
            .iter()
            .map(|&position| {
                get_adjacent_positions(position)
                    .into_iter()
                    .filter(|p| !positions.contains(p))
                    .count()
            })
            .sum();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");

        let min_x = positions.iter().map(|p| p.0).min().unwrap() - 1;
        let min_y = positions.iter().map(|p| p.1).min().unwrap() - 1;
        let min_z = positions.iter().map(|p| p.2).min().unwrap() - 1;
        let max_x = positions.iter().map(|p| p.0).max().unwrap() + 1;
        let max_y = positions.iter().map(|p| p.1).max().unwrap() + 1;
        let max_z = positions.iter().map(|p| p.2).max().unwrap() + 1;

        let lava_cells: HashSet<Point> = positions.iter().copied().collect();
        let mut cells_to_sets: HashMap<Point, Point> = HashMap::new();
        let mut sets_to_cells: HashMap<Point, Vec<Point>> = HashMap::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let cell = (x, y, z);

                    if lava_cells.contains(&cell) {
                        continue;
                    }

                    let sets: Vec<Point> = get_adjacent_positions(cell)
                        .iter()
                        .filter_map(|p| cells_to_sets.get(p))
                        .copied()
                        .unique()
                        .collect();

                    if sets.len() == 1 {
                        let set = sets[0];
                        cells_to_sets.insert(cell, set);
                        sets_to_cells.get_mut(&set).unwrap().push(cell);
                    } else if sets.is_empty() {
                        cells_to_sets.insert(cell, cell);
                        sets_to_cells.insert(cell, vec![cell]);
                    } else {
                        let superset = *sets.iter().min().unwrap();
                        let mut superset_cells = Vec::new();
                        for set in sets {
                            let cells = sets_to_cells.remove(&set).unwrap();
                            for cell in cells {
                                *cells_to_sets.get_mut(&cell).unwrap() = superset;
                                superset_cells.push(cell);
                            }
                        }
                        cells_to_sets.insert(cell, superset);
                        superset_cells.push(cell);
                        sets_to_cells.insert(superset, superset_cells);
                    }
                }
            }
        }

        let outside_set: &[Point] = &sets_to_cells[&(min_x, min_y, min_z)];

        let part_2: usize = positions
            .iter()
            .map(|&position| {
                get_adjacent_positions(position)
                    .into_iter()
                    .filter(|p| outside_set.contains(p))
                    .count()
            })
            .sum();

        println!("Part 2: {part_2}");
    }
}
