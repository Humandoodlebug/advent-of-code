use ansi_term::Colour;
use std::collections::BinaryHeap;
use util::PerfTimer;

extern crate util;

fn input() -> Vec<Vec<i32>> {
    util::get_day_input(15)
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Path {
    cost: i32,
    path: Vec<(usize, usize)>,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        }
    }
}

fn find_min_cost(grid: &[Vec<i32>]) -> i32 {
    let target = (grid.len() - 1, grid[0].len() - 1);
    let mut queue = BinaryHeap::<Path>::new();
    let mut min_costs_per_point = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    min_costs_per_point[0][0] = 0;
    queue.push(Path {
        cost: 0,
        path: vec![(0, 0)],
    });
    loop {
        let path = queue.pop().unwrap();
        let &(x, y) = path.path.last().unwrap();
        if (x, y) == target {
            // print_path(grid, &path.path);
            return path.cost;
        }
        if min_costs_per_point[x][y] < path.cost {
            continue;
        }
        for (i, j) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let x_new = x as i32 + i;
            let y_new = y as i32 + j;
            if x_new < 0 || y_new < 0 || x_new >= grid.len() as i32 || y_new >= grid[0].len() as i32
            {
                continue;
            }
            let x_new = x_new as usize;
            let y_new = y_new as usize;
            let mut new_path = path.clone();
            new_path.path.push((x_new, y_new));
            new_path.cost += grid[x_new][y_new];
            if new_path.cost >= min_costs_per_point[x_new][y_new] {
                continue;
            }
            min_costs_per_point[x_new][y_new] = new_path.cost;
            queue.push(new_path);
        }
    }
}

#[allow(dead_code)]
fn print_path(grid: &[Vec<i32>], path: &[(usize, usize)]) {
    for (x, l) in grid.iter().enumerate() {
        for (y, v) in l.iter().enumerate() {
            if path.contains(&(x, y)) {
                print!("{}", Colour::Green.paint(v.to_string()));
            } else {
                print!("{}", v);
            }
        }
        println!();
    }
}

fn main() {
    let grid = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part1 = find_min_cost(&grid);
        println!("Part 1: {}", part1);
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let large_grid: Vec<Vec<i32>> = (0..5)
            .flat_map(|i| {
                grid.iter()
                    .map(|l| {
                        (0..5)
                            .flat_map(|j| {
                                l.iter()
                                    .map(|v| {
                                        let mut new_v = *v + i + j;
                                        while new_v > 9 {
                                            new_v -= 9
                                        }
                                        new_v
                                    })
                                    .collect::<Vec<i32>>()
                            })
                            .collect()
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .collect();

        let part2 = find_min_cost(&large_grid);
        println!("Part 2: {}", part2)
    }
}
