use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use blizzards::{BlizzardMap, BlizzardMaps};
use contain::SimpleContainer;
use direction::Direction;
use util::PerfTimer;

type Point = (usize, usize);

const START: Point = (0, 1);

mod direction {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Direction {
        Up,
        Right,
        Left,
        Down,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Directions {
        up: bool,
        right: bool,
        left: bool,
        down: bool,
    }

    impl Directions {
        fn get_bool(&self, direction: Direction) -> bool {
            match direction {
                Direction::Up => self.up,
                Direction::Right => self.right,
                Direction::Left => self.left,
                Direction::Down => self.down,
            }
        }

        fn get_bool_mut(&mut self, direction: Direction) -> &mut bool {
            match direction {
                Direction::Up => &mut self.up,
                Direction::Right => &mut self.right,
                Direction::Left => &mut self.left,
                Direction::Down => &mut self.down,
            }
        }

        pub fn new() -> Self {
            Self {
                up: false,
                right: false,
                left: false,
                down: false,
            }
        }

        /// Inserts the given direction, returning whether it was newly inserted.
        pub fn insert(&mut self, direction: Direction) -> bool {
            let d = self.get_bool_mut(direction);

            let newly_inserted = !*d;
            *d = true;
            newly_inserted
        }

        pub fn iter(&self) -> impl Iterator<Item = Direction> + '_ {
            const DIRECTIONS: [Direction; 4] = [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ];
            DIRECTIONS.into_iter().filter(|&d| self.get_bool(d))
        }
    }

    impl Default for Directions {
        fn default() -> Self {
            Self::new()
        }
    }

    impl From<Direction> for Directions {
        fn from(value: Direction) -> Self {
            let mut directions = Self::new();
            directions.insert(value);
            directions
        }
    }
}

struct Parameters {
    blizzards: HashMap<Point, Direction>,
    walls: HashSet<Point>,
    start: Point,
    end: Point,
}

fn input() -> Parameters {
    let raw = util::get_day_input(24);
    let mut blizzards: HashMap<Point, Direction> = HashMap::new();
    let mut walls = HashSet::new();
    for (row, line) in raw.lines().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            match cell {
                '.' => {}
                '#' => {
                    walls.insert((row, col));
                }
                '^' => assert!(blizzards.insert((row, col), Direction::Up).is_none()),
                'v' => assert!(blizzards.insert((row, col), Direction::Down).is_none()),
                '>' => assert!(blizzards.insert((row, col), Direction::Right).is_none()),
                '<' => assert!(blizzards.insert((row, col), Direction::Left).is_none()),
                _ => panic!(),
            }
        }
    }
    let bottom_right = walls.iter().copied().max().unwrap();
    let end = (bottom_right.0, bottom_right.1 - 1);
    assert!(!walls.contains(&START));
    assert!(!walls.contains(&end));

    Parameters {
        blizzards,
        walls,
        start: START,
        end,
    }
}

mod blizzards {
    use std::collections::HashMap;

    use contain::{Container, SimpleContainer};

    use crate::{
        direction::{Direction, Directions},
        Point,
    };

    pub struct BlizzardMap {
        blizzards: HashMap<Point, Directions>,
        height: usize,
        width: usize,
    }

    impl BlizzardMap {
        pub fn new(value: &HashMap<Point, Direction>, width: usize, height: usize) -> Self {
            let mut blizzards: HashMap<Point, Directions> = HashMap::new();
            for (&point, &direction) in value.iter() {
                assert!(blizzards
                    .insert(point, Directions::from(direction))
                    .is_none());
            }

            Self {
                blizzards,
                height,
                width,
            }
        }

        pub fn next(&self) -> Self {
            let mut new_blizzards: HashMap<Point, Directions> = HashMap::new();
            for (&(row, column), directions) in self.blizzards.iter() {
                for direction in directions.iter() {
                    let new_point = match direction {
                        Direction::Up => {
                            if row == 1 {
                                (self.height - 2, column)
                            } else {
                                (row - 1, column)
                            }
                        }
                        Direction::Right => {
                            if column == self.width - 2 {
                                (row, 1)
                            } else {
                                (row, column + 1)
                            }
                        }
                        Direction::Left => {
                            if column == 1 {
                                (row, self.width - 2)
                            } else {
                                (row, column - 1)
                            }
                        }
                        Direction::Down => {
                            if row == self.height - 2 {
                                (1, column)
                            } else {
                                (row + 1, column)
                            }
                        }
                    };
                    new_blizzards
                        .entry(new_point)
                        .or_default()
                        .insert(direction);
                }
            }
            Self {
                blizzards: new_blizzards,
                width: self.width,
                height: self.height,
            }
        }

        pub fn contains(&self, p: &Point) -> bool {
            self.blizzards.contains_key(p)
        }
    }

    pub struct BlizzardMaps<'a> {
        maps: HashMap<usize, &'a BlizzardMap>,
        container: &'a SimpleContainer<'a, BlizzardMap>,
    }

    impl<'a> BlizzardMaps<'a> {
        pub fn new(container: &'a SimpleContainer<'a, BlizzardMap>, initial: BlizzardMap) -> Self {
            let mut maps = HashMap::new();
            let initial = container.put(initial);
            maps.insert(0, initial);
            Self { maps, container }
        }

        pub fn get(&mut self, i: usize) -> &'a BlizzardMap {
            if let Some(&map) = self.maps.get(&i) {
                map
            } else {
                let old_map = self.get(i - 1);
                let new_map = old_map.next();
                let new_map = self.container.put(new_map);
                self.maps.insert(i, new_map);
                new_map
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    position: Point,
    minutes_passed: usize,
    end: Point,
}

impl State {
    #[must_use]
    fn move_up(&self) -> Self {
        Self {
            position: (self.position.0 - 1, self.position.1),
            minutes_passed: self.minutes_passed + 1,
            end: self.end,
        }
    }

    #[must_use]
    fn move_right(&self) -> Self {
        Self {
            position: (self.position.0, self.position.1 + 1),
            minutes_passed: self.minutes_passed + 1,
            end: self.end,
        }
    }

    #[must_use]
    fn move_down(&self) -> Self {
        Self {
            position: (self.position.0 + 1, self.position.1),
            minutes_passed: self.minutes_passed + 1,
            end: self.end,
        }
    }

    #[must_use]
    fn move_left(&self) -> Self {
        Self {
            position: (self.position.0, self.position.1 - 1),
            minutes_passed: self.minutes_passed + 1,
            end: self.end,
        }
    }

    #[must_use]
    fn bide_time(&self) -> Self {
        Self {
            position: (self.position.0, self.position.1),
            minutes_passed: self.minutes_passed + 1,
            end: self.end,
        }
    }

    fn cost_lower_bound(&self) -> usize {
        self.minutes_passed
            + (self.position.0.abs_diff(self.end.0))
            + (self.position.1.abs_diff(self.end.1))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost_lower_bound().cmp(&other.cost_lower_bound()) {
            std::cmp::Ordering::Less => Ordering::Greater,
            std::cmp::Ordering::Equal => (self.position, self.minutes_passed, self.end).cmp(&(
                other.position,
                other.minutes_passed,
                other.end,
            )),
            std::cmp::Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(blizzards: &mut BlizzardMaps, walls: &HashSet<Point>, initial_state: State) -> State {
    let max_row = walls.iter().map(|&(row, _)| row).max().unwrap();
    let mut past_states: HashSet<State> = HashSet::new();
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(initial_state);
    past_states.insert(initial_state);

    loop {
        let state = pq.pop().unwrap();
        if state.position == state.end {
            break state;
        }
        let blizzard_map = blizzards.get(state.minutes_passed + 1);

        if state.position.0 == 0 {
            let new_state = state.move_down();
            let bide_time = state.bide_time();
            if blizzard_map.contains(&new_state.position) && past_states.insert(bide_time) {
                pq.push(state.bide_time());
            } else if past_states.insert(new_state) {
                pq.push(new_state);
            }
        } else if state.position.0 == max_row {
            let new_state = state.move_up();
            let bide_time = state.bide_time();
            if blizzard_map.contains(&new_state.position) && past_states.insert(bide_time) {
                pq.push(state.bide_time());
            } else if past_states.insert(new_state) {
                pq.push(new_state);
            }
        } else {
            for new_state in [
                state.move_up(),
                state.move_right(),
                state.move_down(),
                state.move_left(),
                state.bide_time(),
            ] {
                if !blizzard_map.contains(&new_state.position)
                    && !walls.contains(&new_state.position)
                    && past_states.insert(new_state)
                {
                    pq.push(new_state);
                }
            }
        }
    }
}

fn main() {
    let parameters = input();
    let max_dims = parameters.walls.iter().copied().max().unwrap();
    let height = max_dims.0 + 1;
    let width = max_dims.1 + 1;
    {
        let _timer = PerfTimer::new("Part 1");
        let initial_blizzards = BlizzardMap::new(&parameters.blizzards, width, height);
        let blizzard_container = SimpleContainer::new();
        let mut blizzards = BlizzardMaps::new(&blizzard_container, initial_blizzards);
        let initial_state = State {
            position: parameters.start,
            minutes_passed: 0,
            end: parameters.end,
        };

        let part_1 = solve(&mut blizzards, &parameters.walls, initial_state).minutes_passed;
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let initial_blizzards = BlizzardMap::new(&parameters.blizzards, width, height);
        let blizzard_container = SimpleContainer::new();
        let mut blizzards = BlizzardMaps::new(&blizzard_container, initial_blizzards);
        let initial_state = State {
            position: parameters.start,
            minutes_passed: 0,
            end: parameters.end,
        };

        let journey_1 = solve(&mut blizzards, &parameters.walls, initial_state);
        let journey_2 = solve(
            &mut blizzards,
            &parameters.walls,
            State {
                position: parameters.end,
                end: parameters.start,
                minutes_passed: journey_1.minutes_passed,
            },
        );
        let journey_3 = solve(
            &mut blizzards,
            &parameters.walls,
            State {
                position: parameters.start,
                end: parameters.end,
                minutes_passed: journey_2.minutes_passed,
            },
        );
        let part_2 = journey_3.minutes_passed;
        println!("Part 1: {part_2}");
    }
}
