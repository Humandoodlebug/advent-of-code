#![allow(clippy::cast_sign_loss)]

use regex::Regex;
use util::PerfTimer;

const AREA_WIDTH: i32 = 101;
const AREA_HEIGHT: i32 = 103;

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn moved_many(&self, n: i32) -> Self {
        let mut new = *self;

        new.position.0 = (self.position.0 + self.velocity.0 * n).rem_euclid(AREA_WIDTH);
        new.position.1 = (self.position.1 + self.velocity.1 * n).rem_euclid(AREA_HEIGHT);

        new
    }
}

fn input() -> Vec<Robot> {
    let robot_regex = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    util::get_day_input(14)
        .trim()
        .lines()
        .map(|line| {
            let (_, [p_x, p_y, v_x, v_y]) = robot_regex.captures(line).unwrap().extract();

            Robot {
                position: (p_x.parse().unwrap(), p_y.parse().unwrap()),
                velocity: (v_x.parse().unwrap(), v_y.parse().unwrap()),
            }
        })
        .collect()
}

fn create_grid_after_seconds(robots: &[Robot], seconds: i32) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; AREA_WIDTH as usize]; AREA_HEIGHT as usize];

    for &robot in robots {
        let robot = robot.moved_many(seconds);
        grid[robot.position.1 as usize][robot.position.0 as usize] = true;
    }

    grid
}

fn find_col_sequence_larger_than(grid: &[Vec<bool>], n: usize) -> bool {
    for x in 0..AREA_WIDTH as usize {
        let mut count = 0;
        #[allow(clippy::needless_range_loop)]
        for y in 0..AREA_HEIGHT as usize {
            if grid[y][x] {
                count += 1;
            } else {
                count = 0;
            }

            if count >= n {
                return true;
            }
        }
    }

    false
}

// Used to visualise the grid for investigating the easer egg
#[allow(dead_code)]
fn print_robots_after(robots: &[Robot], seconds: i32, only_interesting: bool) {
    let grid = create_grid_after_seconds(robots, seconds);

    if only_interesting && !find_col_sequence_larger_than(&grid, 20) {
        return;
    }

    println!("After {seconds} seconds:");

    for y in 0..AREA_HEIGHT {
        for x in 0..AREA_WIDTH {
            print!(
                "{}",
                if grid[y as usize][x as usize] {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }

    println!();
    println!();
}

fn main() {
    let robots = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let robots = robots.iter().map(|&robot| robot.moved_many(100));
        let mut top_left_count = 0;
        let mut top_right_count = 0;
        let mut bottom_left_count = 0;
        let mut bottom_right_count = 0;

        for robot in robots {
            if robot.position.0 < AREA_WIDTH / 2 && robot.position.1 < AREA_HEIGHT / 2 {
                top_left_count += 1;
            } else if robot.position.0 > AREA_WIDTH / 2 && robot.position.1 < AREA_HEIGHT / 2 {
                top_right_count += 1;
            } else if robot.position.0 < AREA_WIDTH / 2 && robot.position.1 > AREA_HEIGHT / 2 {
                bottom_left_count += 1;
            } else if robot.position.0 > AREA_WIDTH / 2 && robot.position.1 > AREA_HEIGHT / 2 {
                bottom_right_count += 1;
            }
        }

        let part_1 = top_left_count * top_right_count * bottom_left_count * bottom_right_count;

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");

        let robots = robots.clone();

        for i in 0.. {
            // print_robots_after(&robots, i, true);

            let grid = create_grid_after_seconds(&robots, i);
            if find_col_sequence_larger_than(&grid, 20) {
                println!("Part 2: {i}");
                break;
            }
        }
    }
}
