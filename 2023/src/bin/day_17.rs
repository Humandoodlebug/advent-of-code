use priority_queue::PriorityQueue;
use util::PerfTimer;

fn input() -> Vec<Vec<u32>> {
    util::get_day_input(17)
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn mv(
        self,
        rows: usize,
        cols: usize,
        row: usize,
        col: usize,
        len: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if row >= len {
                    Some((row - len, col))
                } else {
                    None
                }
            }
            Direction::Right => {
                if col + len < cols {
                    Some((row, col + len))
                } else {
                    None
                }
            }
            Direction::Down => {
                if row + len < rows {
                    Some((row + len, col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if col >= len {
                    Some((row, col - len))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    row: usize,
    col: usize,
    direction: Direction,
    path_cost: u32,
}

fn mv(map: &[Vec<u32>], min_moves: usize, max_moves: usize, state: State) -> Vec<State> {
    let mut next_states = Vec::new();
    let rows = map.len();
    let cols = map[0].len();
    let left = state.direction.turn_left();
    let right = state.direction.turn_right();
    let mut left_cost = state.path_cost;
    let mut right_cost = state.path_cost;
    for len in 1..=max_moves {
        if let Some((next_row, next_col)) = left.mv(rows, cols, state.row, state.col, len) {
            left_cost += map[next_row][next_col];
            if len >= min_moves {
                next_states.push(State {
                    row: next_row,
                    col: next_col,
                    path_cost: left_cost,
                    direction: left,
                })
            }
        }
        if let Some((next_row, next_col)) = right.mv(rows, cols, state.row, state.col, len) {
            right_cost += map[next_row][next_col];
            if len >= min_moves {
                next_states.push(State {
                    row: next_row,
                    col: next_col,
                    path_cost: right_cost,
                    direction: right,
                })
            }
        }
    }
    next_states
}

fn calculate_priority(map: &[Vec<u32>], state: &State) -> u32 {
    u32::MAX
        - state.path_cost
        - ((map.len() - state.row - 1) + (map[0].len() - state.col - 1)) as u32
}

fn hash_position(map: &[Vec<u32>], state: &State) -> usize {
    (state.row * map[0].len() + state.col) * 4 + state.direction as usize
}

fn run(map: &[Vec<u32>], min_moves: usize, max_moves: usize) -> u32 {
    let initial_states: Vec<State> = [Direction::Right, Direction::Down]
        .into_iter()
        .map(|direction| State {
            row: 0,
            col: 0,
            direction,
            path_cost: 0,
        })
        .collect();
    let mut been = vec![false; map.len() * map[0].len() * 4];
    let mut to_process = PriorityQueue::new();
    for s in initial_states {
        to_process.push(s, calculate_priority(map, &s));
    }
    loop {
        let (state, _) = to_process.pop().unwrap();
        if state.row == map.len() - 1 && state.col == map[0].len() - 1 {
            break state.path_cost;
        }
        if been[hash_position(map, &state)] {
            continue;
        };
        been[hash_position(map, &state)] = true;
        let next_states = mv(map, min_moves, max_moves, state);
        for next_state in next_states {
            if !been[hash_position(map, &next_state)] {
                to_process.push(next_state, calculate_priority(map, &next_state));
            }
        }
    }
}

fn main() {
    let map = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = run(&map, 1, 3);
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = run(&map, 4, 10);
        println!("Part 2: {part_2}");
    }
}
