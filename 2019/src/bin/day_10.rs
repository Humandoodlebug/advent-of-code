use std::f64::consts::PI;

use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<Vec<bool>> {
    let raw = util::get_day_input(10);
    let mut grid = Vec::new();
    for line in raw.trim().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        grid.push(row);
    }
    grid
}

fn is_line_of_sight(grid: &[Vec<bool>], pos_a: (usize, usize), pos_b: (usize, usize)) -> bool {
    let pos_b_from_a = (
        (pos_b.0 as isize) - (pos_a.0 as isize),
        (pos_b.1 as isize) - (pos_a.1 as isize),
    );
    for (asteroid_y, asteroid_row) in grid.iter().enumerate() {
        for (asteroid_x, &asteroid_cell) in asteroid_row.iter().enumerate() {
            if asteroid_cell
                && pos_a != (asteroid_x, asteroid_y)
                && pos_b != (asteroid_x, asteroid_y)
            {
                let asteroid_pos_from_a = (
                    (asteroid_x as isize) - (pos_a.0 as isize),
                    (asteroid_y as isize) - (pos_a.1 as isize),
                );
                if asteroid_pos_from_a.0 * pos_b_from_a.1 == asteroid_pos_from_a.1 * pos_b_from_a.0
                    && ((asteroid_pos_from_a.0 >= 0 && pos_b_from_a.0 >= 0)
                        || (asteroid_pos_from_a.0 <= 0 && pos_b_from_a.0 <= 0))
                    && ((asteroid_pos_from_a.1 >= 0 && pos_b_from_a.1 >= 0)
                        || (asteroid_pos_from_a.1 <= 0 && pos_b_from_a.1 <= 0))
                    && (asteroid_pos_from_a.0.abs() < pos_b_from_a.0.abs()
                        || asteroid_pos_from_a.1.abs() < pos_b_from_a.1.abs())
                {
                    return false;
                }
            }
        }
    }
    true
}

fn count_asteroids_visible_from(grid: &[Vec<bool>], pos: (usize, usize)) -> usize {
    let mut count = 0;
    for (asteroid_y, asteroid_row) in grid.iter().enumerate() {
        for (asteroid_x, &asteroid_cell) in asteroid_row.iter().enumerate() {
            if asteroid_cell
                && pos != (asteroid_x, asteroid_y)
                && is_line_of_sight(grid, pos, (asteroid_x, asteroid_y))
            {
                count += 1;
            }
        }
    }
    count
}

fn arctan(opposite: f64, adjacent: f64) -> f64 {
    let mut basic = opposite.atan2(adjacent);
    if basic < 0f64 {
        basic += 2f64 * PI;
    }
    basic
}

fn main() {
    let grid = input();

    let mut station_coordinates = (0, 0);
    {
        let _timer = PerfTimer::new("Part 1");
        let mut asteroids_in_view = 0;
        for (station_y, station_row) in grid.iter().enumerate() {
            for (station_x, &station_cell) in station_row.iter().enumerate() {
                if station_cell {
                    let new_in_view = count_asteroids_visible_from(&grid, (station_x, station_y));
                    if new_in_view >= asteroids_in_view {
                        asteroids_in_view = new_in_view;
                        station_coordinates = (station_x, station_y);
                    }
                }
            }
        }

        println!("Part 1: {asteroids_in_view}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let asteroids: Vec<(usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(
                    move |(x, &station_cell)| {
                        if station_cell {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect();
        let ordered_asteroids = asteroids.iter().copied().sorted_by(|&(x1, y1), &(x2, y2)| {
            arctan(
                (x1 as isize - station_coordinates.0 as isize) as f64,
                -(y1 as isize - station_coordinates.1 as isize) as f64,
            )
            .total_cmp(&arctan(
                (x2 as isize - station_coordinates.0 as isize) as f64,
                -(y2 as isize - station_coordinates.1 as isize) as f64,
            ))
        });
        let mut asteroid_groups = Vec::new();
        let mut asteroid_group = Vec::new();
        let mut ast_x = 0;
        let mut ast_y = 0;
        for (x, y) in ordered_asteroids {
            if (x, y) == station_coordinates {
                continue;
            }
            if asteroid_group.is_empty() {
                asteroid_group.push((x, y));
                ast_x = x as isize - station_coordinates.0 as isize;
                ast_y = y as isize - station_coordinates.1 as isize;
            } else {
                let st_x = x as isize - station_coordinates.0 as isize;
                let st_y = y as isize - station_coordinates.1 as isize;
                if ast_x * st_y == ast_y * st_x
                    && ((ast_x >= 0 && st_x >= 0) || (ast_x <= 0 && st_x <= 0))
                    && ((ast_y >= 0 && st_y >= 0) || (ast_y <= 0 && st_y <= 0))
                {
                    asteroid_group.push((x, y));
                } else {
                    asteroid_groups.push(asteroid_group);
                    asteroid_group = vec![(x, y)];
                    ast_x = st_x;
                    ast_y = st_y;
                }
            }
        }
        asteroid_groups.push(asteroid_group);

        for group in &mut asteroid_groups {
            group.sort_by_key(|&(x, y)| {
                -((x as isize - station_coordinates.0 as isize).abs().pow(2)
                    + (y as isize - station_coordinates.1 as isize).abs().pow(2))
            });
        }

        let mut asteroid_count = 0;
        let (x, y) = 'outer: loop {
            for group in &mut asteroid_groups {
                if let Some(a) = group.pop() {
                    asteroid_count += 1;
                    if asteroid_count == 200 {
                        break 'outer a;
                    }
                }
            }
        };
        let part_2 = x * 100 + y;
        println!("Part 2: {part_2}");
    }
}
