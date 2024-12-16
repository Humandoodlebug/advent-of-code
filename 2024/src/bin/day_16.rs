#![allow(clippy::match_on_vec_items, clippy::type_complexity)]

use std::collections::{hash_map::Entry as HashMapEntry, BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

fn input() -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;

    let map = util::get_day_input(16)
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'S' => {
                        start = Some((x, y));
                        Tile::Empty
                    }
                    'E' => {
                        end = Some((x, y));
                        Tile::Empty
                    }
                    _ => panic!("invalid character in input map"),
                })
                .collect()
        })
        .collect();

    (
        map,
        start.expect("start not specified in input map"),
        end.expect("end not specified in input map"),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn mv(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1),
        }
    }

    fn turn_left_then_mv(self, pos: (usize, usize)) -> ((usize, usize), Direction) {
        match self {
            Direction::North => ((pos.0 - 1, pos.1), Direction::West),
            Direction::East => ((pos.0, pos.1 - 1), Direction::North),
            Direction::South => ((pos.0 + 1, pos.1), Direction::East),
            Direction::West => ((pos.0, pos.1 + 1), Direction::South),
        }
    }

    fn turn_right_then_mv(self, pos: (usize, usize)) -> ((usize, usize), Direction) {
        match self {
            Direction::North => ((pos.0 + 1, pos.1), Direction::East),
            Direction::East => ((pos.0, pos.1 + 1), Direction::South),
            Direction::South => ((pos.0 - 1, pos.1), Direction::West),
            Direction::West => ((pos.0, pos.1 - 1), Direction::North),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: (usize, usize),
    direction: Direction,
    score: u64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    #[allow(clippy::cast_possible_wrap)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (-(self.score as i64), self.position, self.direction).cmp(&(
            -(other.score as i64),
            other.position,
            other.direction,
        ))
    }
}

fn next_states(map: &[Vec<Tile>], state: State) -> Vec<State> {
    let mut new_states = Vec::new();

    let forward = state.direction.mv(state.position);
    let left = state.direction.turn_left_then_mv(state.position);
    let right = state.direction.turn_right_then_mv(state.position);

    if map[forward.1][forward.0] == Tile::Empty {
        new_states.push(State {
            position: forward,
            direction: state.direction,
            score: state.score + 1,
        });
    }

    for (new_position, new_direction) in [left, right] {
        if map[new_position.1][new_position.0] == Tile::Empty {
            new_states.push(State {
                position: new_position,
                direction: new_direction,
                score: state.score + 1001,
            });
        }
    }

    new_states
}

fn main() {
    let (map, start, end) = input();

    let min_score;

    {
        let _timer = PerfTimer::new("Part 1");

        let mut states = BinaryHeap::new();
        let mut past_states = HashSet::new();

        states.push(State {
            position: start,
            direction: Direction::East,
            score: 0,
        });

        loop {
            let state = states.pop().unwrap();

            if !past_states.insert((state.position, state.direction)) {
                continue;
            }

            if state.position == end {
                min_score = state.score;
                println!("Part 1: {}", state.score);
                break;
            }

            for new_state in next_states(&map, state) {
                states.push(new_state);
            }
        }
    }

    {
        let _timer = PerfTimer::new("Part 2");

        let mut states = BinaryHeap::new();
        let mut shortest_paths: HashMap<
            ((usize, usize), Direction),
            (HashSet<(usize, usize)>, u64),
        > = HashMap::new();
        let mut past_states = HashSet::new();

        states.push(State {
            position: start,
            direction: Direction::East,
            score: 0,
        });

        shortest_paths.insert((start, Direction::East), (HashSet::from_iter([start]), 0));

        while let Some(state) = states.pop() {
            if !past_states.insert((state.position, state.direction)) {
                continue;
            }

            let (shortest_paths_tiles, _) = shortest_paths
                .get(&(state.position, state.direction))
                .unwrap()
                .clone();

            for new_state in next_states(&map, state) {
                let mut new_shortest_path = shortest_paths_tiles.clone();
                new_shortest_path.insert(new_state.position);
                match shortest_paths.entry((new_state.position, new_state.direction)) {
                    HashMapEntry::Occupied(mut entry) => {
                        let (path, score) = entry.get_mut();
                        match new_state.score.cmp(score) {
                            std::cmp::Ordering::Less => {
                                *path = new_shortest_path;
                                *score = new_state.score;
                            }
                            std::cmp::Ordering::Equal => {
                                path.extend(new_shortest_path);
                            }
                            std::cmp::Ordering::Greater => {}
                        }
                    }
                    HashMapEntry::Vacant(entry) => {
                        entry.insert((new_shortest_path, new_state.score));
                    }
                }
                states.push(new_state);
            }
        }

        let part_2 = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .flat_map(|direction| {
            let Some((path_locations, score)) = shortest_paths.get(&(end, direction)) else {
                return Vec::new();
            };
            if *score == min_score {
                path_locations.iter().collect_vec()
            } else {
                Vec::new()
            }
        })
        .count();

        println!("Part 2: {part_2}");
    }
}
