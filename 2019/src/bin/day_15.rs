use std::collections::{HashSet, VecDeque};

use util::{intcode::State, PerfTimer};

type Point = (i64, i64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

enum StatusCode {
    /// Droid hit a wall (didn't move)
    HitWall,
    /// Droid moved into empty space
    Moved,
    /// Droid moved and found oxygen
    FoundOxygen,
}

impl From<i128> for StatusCode {
    fn from(value: i128) -> Self {
        match value {
            0 => StatusCode::HitWall,
            1 => StatusCode::Moved,
            2 => StatusCode::FoundOxygen,
            o => panic!("invalid status code: {o}"),
        }
    }
}

impl Direction {
    fn command(&self) -> i128 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn mv(&self, (row, col): Point) -> Point {
        match self {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
            Direction::East => (row, col + 1),
        }
    }
}

#[derive(Clone, Debug)]
struct RobotState {
    path: Vec<Direction>,
    position: Point,
    state: State,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchFor {
    Oxygen,
    LongestPath,
}

fn find_path(initial_state: RobotState, search_for: SearchFor) -> RobotState {
    use Direction::*;
    let mut discovered: HashSet<Point> = HashSet::new();
    discovered.insert(initial_state.position);
    let mut states = VecDeque::new();
    let mut last_state = initial_state.clone();
    states.push_back(initial_state);
    loop {
        let state = match states.pop_front() {
            Some(s) => {
                last_state = s.clone();
                s
            }
            None => match search_for {
                SearchFor::Oxygen => panic!(),
                SearchFor::LongestPath => {
                    return last_state;
                }
            },
        };
        for direction in [North, South, West, East] {
            let new_pos = direction.mv(state.position);
            if !discovered.insert(new_pos) {
                continue;
            }
            let mut new_state = state.state.clone();
            let status_code =
                StatusCode::from(new_state.run_one_in_one_out(direction.command()).unwrap());
            match status_code {
                StatusCode::HitWall => {}
                StatusCode::Moved => {
                    let mut new_path = state.path.clone();
                    new_path.push(direction);
                    states.push_back(RobotState {
                        path: new_path,
                        position: new_pos,
                        state: new_state,
                    })
                }
                StatusCode::FoundOxygen if search_for == SearchFor::Oxygen => {
                    let mut new_path = state.path;
                    new_path.push(direction);
                    return RobotState {
                        path: new_path,
                        position: new_pos,
                        state: new_state,
                    };
                }
                StatusCode::FoundOxygen => {
                    panic!();
                }
            }
        }
    }
}

fn main() {
    let input: Vec<i128> = util::get_day_input(15)
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    {
        let _timer = PerfTimer::new("Part 1");
        let mem = input.clone();
        let path = find_path(
            RobotState {
                path: Vec::new(),
                position: (0, 0),
                state: State::new(mem),
            },
            SearchFor::Oxygen,
        )
        .path;
        let part_1 = path.len();
        println!("Part 1: {part_1}");

        // Scrap exploring, here's an idea: keep expanding the shortest path, never going to a position we've already
        // been to, until we find oxygen. Keep track of all the states simultaneously.
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mem = input.clone();
        let at_oxygen = find_path(
            RobotState {
                path: Vec::new(),
                position: (0, 0),
                state: State::new(mem),
            },
            SearchFor::Oxygen,
        );
        let path = find_path(
            RobotState {
                path: Vec::new(),
                ..at_oxygen
            },
            SearchFor::LongestPath,
        )
        .path;
        let part_2 = path.len();
        println!("Part 2: {part_2}");
    }
}
