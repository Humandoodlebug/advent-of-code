use std::collections::HashSet;

use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn move_in_direction(pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        Direction::Right => Some((pos.0 + 1, pos.1)),
        Direction::Down => Some((pos.0, pos.1 + 1)),
        Direction::Left => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
    }
}

fn input() -> (Vec<Vec<bool>>, (usize, usize), Direction) {
    let raw = util::get_day_input(6);
    let mut guard_position = None;
    let mut guard_direction = None;

    let grid = raw
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if ['^', 'v', '<', '>'].contains(&c) {
                        assert!(
                            guard_position.is_none() && guard_direction.is_none(),
                            "multiple guards found"
                        );
                        guard_position = Some((x, y));
                        guard_direction = match c {
                            '^' => Some(Direction::Up),
                            'v' => Some(Direction::Down),
                            '<' => Some(Direction::Left),
                            '>' => Some(Direction::Right),
                            _ => unreachable!(),
                        };
                    }
                    c == '#'
                })
                .collect()
        })
        .collect();

    (
        grid,
        guard_position.expect("guard position not found"),
        guard_direction.unwrap(),
    )
}

fn main() {
    let (grid, guard_position, guard_direction) = input();

    let guard_locations = {
        let _timer = PerfTimer::new("Part 1");

        let mut guard_locations = HashSet::new();
        let mut guard_position = guard_position;
        let mut guard_direction = guard_direction;

        loop {
            guard_locations.insert(guard_position);

            let Some(new_position) = move_in_direction(guard_position, guard_direction) else {
                break;
            };
            if new_position.1 >= grid.len() || new_position.0 >= grid[0].len() {
                break;
            }

            if grid[new_position.1][new_position.0] {
                guard_direction = guard_direction.turn_right();
            } else {
                guard_position = new_position;
            }
        }

        println!("Part 1: {}", guard_locations.len());

        guard_locations
    };

    {
        let _timer = PerfTimer::new("Part 2");
        let mut loop_count: usize = 0;

        for obstruction_location in guard_locations {
            let mut guard_locations = HashSet::new();
            let mut guard_position = guard_position;
            let mut guard_direction = guard_direction;

            loop {
                if !guard_locations.insert((guard_position, guard_direction)) {
                    loop_count += 1;
                    break;
                }

                let Some(new_position) = move_in_direction(guard_position, guard_direction) else {
                    break;
                };
                if new_position.1 >= grid.len() || new_position.0 >= grid[0].len() {
                    break;
                }

                if grid[new_position.1][new_position.0] || new_position == obstruction_location {
                    guard_direction = guard_direction.turn_right();
                } else {
                    guard_position = new_position;
                }
            }
        }

        println!("Part 2: {loop_count}");
    }
}
