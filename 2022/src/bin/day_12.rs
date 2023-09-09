use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use util::PerfTimer;

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct State {
    path_len: usize,
    pos: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn input() -> (Vec<Vec<u8>>, Point, Point) {
    let raw = util::get_day_input(12);
    let mut start_pos = None;
    let mut end_pos = None;
    let grid = raw
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c.is_ascii_lowercase() {
                        c as u8 - b'a'
                    } else if c == 'S' {
                        start_pos = Some((i, j));
                        0
                    } else if c == 'E' {
                        end_pos = Some((i, j));
                        b'z' - b'a'
                    } else {
                        panic!("Unrecognised input character {c:?} at ({i},{j})");
                    }
                })
                .collect()
        })
        .collect();
    (grid, start_pos.unwrap(), end_pos.unwrap())
}

fn path_lower_bound(pos: Point, target: Point) -> usize {
    pos.0.abs_diff(target.0) + pos.1.abs_diff(target.1)
}

fn find_shortest_path(grid: &Vec<Vec<u8>>, start_pos: Point, end_pos: Point) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    let initial_lb = path_lower_bound(start_pos, end_pos);
    let initial_state = State {
        pos: start_pos,
        path_len: 0,
        cost: initial_lb,
    };
    states.push(initial_state);
    loop {
        if states.is_empty() {
            break None;
        }
        let state = states.pop().unwrap();
        if !visited.insert(state.pos) {
            continue;
        }
        if state.pos == end_pos {
            break Some(state.path_len);
        }
        let (i, j) = state.pos;

        let height = grid[i][j];
        if i > 0 && grid[i - 1][j] <= height + 1 {
            let new_pos = (i - 1, j);
            if !visited.contains(&new_pos) {
                let new_path_len = state.path_len + 1;
                let new_lb = path_lower_bound(new_pos, end_pos) + new_path_len;
                states.push(State {
                    path_len: state.path_len + 1,
                    pos: new_pos,
                    cost: new_lb,
                });
            }
        }
        if j > 0 && grid[i][j - 1] <= height + 1 {
            let new_pos = (i, j - 1);
            if !visited.contains(&new_pos) {
                let new_path_len = state.path_len + 1;
                let new_lb = path_lower_bound(new_pos, end_pos) + new_path_len;
                states.push(State {
                    path_len: state.path_len + 1,
                    pos: new_pos,
                    cost: new_lb,
                });
            }
        }
        if i < grid.len() - 1 && grid[i + 1][j] <= height + 1 {
            let new_pos = (i + 1, j);
            if !visited.contains(&new_pos) {
                let new_path_len = state.path_len + 1;
                let new_lb = path_lower_bound(new_pos, end_pos) + new_path_len;
                states.push(State {
                    path_len: state.path_len + 1,
                    pos: new_pos,
                    cost: new_lb,
                });
            }
        }
        if j < grid[0].len() - 1 && grid[i][j + 1] <= height + 1 {
            let new_pos = (i, j + 1);
            if !visited.contains(&new_pos) {
                let new_path_len = state.path_len + 1;
                let new_lb = path_lower_bound(new_pos, end_pos) + new_path_len;
                states.push(State {
                    path_len: state.path_len + 1,
                    pos: new_pos,
                    cost: new_lb,
                });
            }
        }
    }
}

fn main() {
    let (grid, start_pos, end_pos) = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = find_shortest_path(&grid, start_pos, end_pos).unwrap();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let start_points: Vec<Point> = grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &h)| if h == 0 { Some((i, j)) } else { None })
                    .collect::<Vec<Point>>()
            })
            .collect();
        let part_2 = start_points
            .into_iter()
            .filter_map(|start_pos| find_shortest_path(&grid, start_pos, end_pos))
            .min()
            .unwrap();
        println!("Part 2: {part_2}");
    }
}
