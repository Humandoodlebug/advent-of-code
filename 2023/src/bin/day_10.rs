use std::collections::HashMap;

use util::PerfTimer;

#[derive(Clone, Copy, Debug, Default)]
struct Pipe {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Pipe {
    fn has_pipe(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => self.north,
            Direction::East => self.east,
            Direction::South => self.south,
            Direction::West => self.west,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_one(self, (row, column): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (row - 1, column),
            Direction::East => (row, column + 1),
            Direction::South => (row + 1, column),
            Direction::West => (row, column - 1),
        }
    }
    fn mirror(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn input() -> (Vec<Vec<Pipe>>, (usize, usize)) {
    let mut start = None;
    (
        util::get_day_input(10)
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column, c)| match c {
                        '|' => Pipe {
                            north: true,
                            east: false,
                            south: true,
                            west: false,
                        },
                        '-' => Pipe {
                            north: false,
                            east: true,
                            south: false,
                            west: true,
                        },
                        'L' => Pipe {
                            north: true,
                            east: true,
                            south: false,
                            west: false,
                        },
                        'J' => Pipe {
                            north: true,
                            east: false,
                            south: false,
                            west: true,
                        },
                        '7' => Pipe {
                            north: false,
                            east: false,
                            south: true,
                            west: true,
                        },
                        'F' => Pipe {
                            north: false,
                            east: true,
                            south: true,
                            west: false,
                        },
                        '.' => Pipe::default(),
                        'S' => {
                            start = Some((row, column));
                            Pipe {
                                north: true,
                                east: true,
                                south: true,
                                west: true,
                            }
                        }
                        _ => panic!("unexpected char: {}", c),
                    })
                    .collect()
            })
            .collect(),
        start.unwrap(),
    )
}

#[derive(Clone, Copy, Debug)]
struct State {
    position: (usize, usize), // row, column
    last_move: Option<Direction>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchState {
    Outside,
    Entering,
    Inside,
    Leaving,
}

fn move_state(map: &[Vec<Pipe>], state: State) -> Vec<State> {
    let mut new_states = Vec::new();
    for direction in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        if state.last_move.map(Direction::mirror) == Some(direction)
            || state.position.0 == 0 && direction == Direction::North
            || state.position.1 == 0 && direction == Direction::West
        {
            continue;
        }
        if map[state.position.0][state.position.1].has_pipe(direction) {
            let new_position = direction.move_one(state.position);
            if new_position.0 == map.len() || new_position.1 == map[0].len() {
                continue;
            }
            if map[new_position.0][new_position.1].has_pipe(direction.mirror()) {
                new_states.push(State {
                    position: new_position,
                    last_move: Some(direction),
                })
            }
        }
    }
    new_states
}

fn main() {
    let (map, start) = input();
    let _timer = PerfTimer::new("Both parts");

    // Part 1
    let initial_state = State {
        position: start,
        last_move: None,
    };
    let mut states = vec![initial_state];
    let mut been = HashMap::new();
    been.insert(initial_state.position, 0);
    'a: for i in 1.. {
        let mut next_states = Vec::new();
        for state in states {
            let new_states = move_state(&map, state);
            for new_state in new_states {
                if let Some(moves) = been.get(&new_state.position) {
                    println!("Part 1: {moves}");
                    break 'a;
                } else {
                    been.insert(new_state.position, i);
                    next_states.push(new_state);
                }
            }
        }
        states = next_states;
    }

    // Part 2
    let mut state = SearchState::Outside;
    let mut count: u64 = 0;
    let start_pipe = Pipe {
        north: start.0 > 0 && map[start.0 - 1][start.1].south,
        east: start.1 < map[0].len() - 1 && map[start.0][start.1 + 1].west,
        south: start.0 < map.len() - 1 && map[start.0 + 1][start.1].north,
        west: start.1 > 0 && map[start.0][start.1 - 1].east,
    };
    let map = {
        let mut map = map;
        map[start.0][start.1] = start_pipe;
        map
    };
    for (row, line) in map.iter().enumerate() {
        for (col, pipe) in line.iter().enumerate() {
            if been.contains_key(&(row, col)) {
                if pipe.east {
                    if pipe.west {
                        assert!(state == SearchState::Entering || state == SearchState::Leaving);
                    } else if state == SearchState::Outside {
                        if pipe.north {
                            state = SearchState::Leaving;
                        } else if pipe.south {
                            state = SearchState::Entering;
                        } else {
                            panic!();
                        }
                    } else if state == SearchState::Inside {
                        if pipe.north {
                            state = SearchState::Entering;
                        } else if pipe.south {
                            state = SearchState::Leaving;
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    state = match state {
                        SearchState::Outside => SearchState::Inside,
                        SearchState::Entering => {
                            if pipe.north {
                                SearchState::Inside
                            } else if pipe.south {
                                SearchState::Outside
                            } else {
                                panic!();
                            }
                        }
                        SearchState::Leaving => {
                            if pipe.south {
                                SearchState::Inside
                            } else {
                                SearchState::Outside
                            }
                        }
                        SearchState::Inside => SearchState::Outside,
                    }
                }
            } else {
                assert!(state == SearchState::Inside || state == SearchState::Outside);
                if state == SearchState::Inside {
                    count += 1;
                }
            }
        }
        assert_eq!(state, SearchState::Outside)
    }
    println!("Part 2: {count}")
}
