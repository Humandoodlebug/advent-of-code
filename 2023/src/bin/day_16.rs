use std::collections::HashSet;

use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,          // .
    ForwardMirror,  // /
    BackwardMirror, // \
    H2VSplitter,    // |
    V2HSplitter,    // -
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn mv(
        self,
        grid_height: usize,
        grid_width: usize,
        row: usize,
        col: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Direction::Right => {
                if col < grid_width - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if row < grid_height - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
        }
    }
}

impl Tile {
    fn act(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Tile::Empty => vec![direction],
            Tile::ForwardMirror => vec![match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            }],
            Tile::BackwardMirror => vec![match direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            }],
            Tile::H2VSplitter => match direction {
                Direction::Up | Direction::Down => vec![direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
            Tile::V2HSplitter => match direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![direction],
            },
        }
    }
}

fn input() -> Vec<Vec<Tile>> {
    util::get_day_input(16)
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::ForwardMirror,
                    '\\' => Tile::BackwardMirror,
                    '|' => Tile::H2VSplitter,
                    '-' => Tile::V2HSplitter,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    row: usize,
    col: usize,
    direction: Direction,
}

fn count_energised(grid: &[Vec<Tile>], initial_state: State) -> usize {
    let mut energised = HashSet::new();
    energised.insert((initial_state.row, initial_state.col));
    let mut past_states = HashSet::new();
    let mut states_to_process = vec![initial_state];
    while let Some(state) = states_to_process.pop() {
        if !past_states.insert(state) {
            continue;
        }
        energised.insert((state.row, state.col));
        let new_directions = grid[state.row][state.col].act(state.direction);
        let new_states = new_directions.into_iter().filter_map(|direction| {
            let (row, col) = direction.mv(grid.len(), grid[0].len(), state.row, state.col)?;
            Some(State {
                row,
                col,
                direction,
            })
        });
        states_to_process.extend(new_states);
    }
    energised.len()
}

fn main() {
    let grid = input();
    {
        let _timer = PerfTimer::new("Part 1");

        let part_1 = count_energised(
            &grid,
            State {
                row: 0,
                col: 0,
                direction: Direction::Right,
            },
        );
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let initial_states = (0..grid.len())
            .map(|row| State {
                row,
                col: 0,
                direction: Direction::Right,
            })
            .chain((0..grid[0].len()).map(|col| State {
                row: 0,
                col,
                direction: Direction::Down,
            }))
            .chain((0..grid.len()).map(|row| State {
                row,
                col: grid[0].len() - 1,
                direction: Direction::Left,
            }))
            .chain((0..grid[0].len()).map(|col| State {
                row: grid.len() - 1,
                col,
                direction: Direction::Up,
            }));
        let part_2 = initial_states
            .map(|s| count_energised(&grid, s))
            .max()
            .unwrap();
        println!("Part 2: {part_2}");
    }
}
