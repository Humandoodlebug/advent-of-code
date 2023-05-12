#![feature(map_try_insert)]

use std::collections::{HashMap, HashSet};

use util::PerfTimer;

type Point = (i64, i64);

const WIDTH: i64 = 7;

#[derive(Clone, Copy, Debug)]
enum Movement {
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct RoughState<const N: usize> {
    movement_index: usize,
    shape_index: usize,
    positions_sort_of: [Point; N],
}

fn input() -> Vec<Movement> {
    util::get_day_input(17)
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Movement::Left,
            '>' => Movement::Right,
            c => panic!("found unexpected character {c:?} in input"),
        })
        .collect()
}

fn add_point((a_x, a_y): Point, (b_x, b_y): Point) -> Point {
    (a_x + b_x, a_y + b_y)
}

fn simulate_falling_rock(
    rock: &[Point],
    rocks: &mut HashSet<Point>,
    movements: &mut impl Iterator<Item = Movement>,
    tower_height: &mut i64,
) -> Point {
    let mut rock_offset: Point = (2, *tower_height + 3);
    assert!(rock
        .iter()
        .map(|&p| add_point(p, rock_offset))
        .all(|r| !rocks.contains(&r)));

    loop {
        // move left/right
        let new_rock_offset = match movements.next().unwrap() {
            Movement::Left => add_point(rock_offset, (-1, 0)),
            Movement::Right => add_point(rock_offset, (1, 0)),
        };

        if new_rock_offset.0 >= 0
            && rock
                .iter()
                .map(|&p| add_point(p, new_rock_offset))
                .all(|p| p.0 < WIDTH && !rocks.contains(&p))
        {
            rock_offset = new_rock_offset;
        }

        // move down
        let new_rock_offset = add_point(rock_offset, (0, -1));

        if new_rock_offset.1 < 0
            || rock
                .iter()
                .map(|&p| add_point(p, new_rock_offset))
                .any(|p| rocks.contains(&p))
        {
            break;
        }

        rock_offset = new_rock_offset;
    }

    for p in rock.iter().map(|&p| add_point(p, rock_offset)) {
        *tower_height = (*tower_height).max(p.1 + 1);
        rocks.insert(p);
    }
    rock_offset
}

fn main() {
    let shapes: [&[Point]; 5] = [
        &[(0, 0), (1, 0), (2, 0), (3, 0)],         // horizontal line
        &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // + shape
        &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // backwards L shape
        &[(0, 0), (0, 1), (0, 2), (0, 3)],         // vertical line
        &[(0, 0), (1, 0), (0, 1), (1, 1)],         // square
    ];

    let movements = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut movements = movements.iter().cycle().copied();
        let mut rocks: HashSet<Point> = HashSet::new();
        let mut tower_height = 0;
        for rock in shapes.iter().cycle().copied().take(2022) {
            simulate_falling_rock(rock, &mut rocks, &mut movements, &mut tower_height);
        }
        let part_1 = tower_height;
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        const LOOK_BACK_LENGTH: usize = 10;
        let mut movements = movements.iter().copied().enumerate().cycle().peekable();
        let mut rocks: HashSet<Point> = HashSet::new();
        let mut tower_height = 0;
        let mut tower_heights = Vec::new();
        let mut rough_states: HashMap<RoughState<LOOK_BACK_LENGTH>, i64> = HashMap::new();
        let mut rock_positions = Vec::new();
        for (shape_index, rock) in shapes.iter().copied().enumerate().cycle() {
            let rock_pos = simulate_falling_rock(
                rock,
                &mut rocks,
                &mut (&mut movements).map(|(_i, m)| m),
                &mut tower_height,
            );
            rock_positions.push(rock_pos);
            tower_heights.push(tower_height);
            let movement_index = movements.peek().unwrap().0;
            let mut positions_sort_of = [(0, 0); LOOK_BACK_LENGTH];
            let first_height =
                rock_positions[rock_positions.len() - LOOK_BACK_LENGTH.min(rock_positions.len())].1;
            for (i, position_sort_of) in positions_sort_of.iter_mut().enumerate() {
                if rock_positions.len() >= LOOK_BACK_LENGTH - i {
                    *position_sort_of = add_point(
                        rock_positions[rock_positions.len() - LOOK_BACK_LENGTH + i],
                        (0, -first_height),
                    );
                }
            }

            let rough_state = RoughState {
                movement_index,
                shape_index,
                positions_sort_of,
            };

            if rough_states
                .try_insert(rough_state.clone(), rough_states.len() as i64)
                .is_err()
            {
                // this means we completed one cycle on step current - LOOK_BACK_LENGTH
                let first_cycle_state = rough_states.get(&rough_state).unwrap();
                let first_cycle_step = first_cycle_state - (LOOK_BACK_LENGTH - 1) as i64;
                let second_cycle_state = rough_states.len() as i64;
                let last_cycle_step = second_cycle_state - LOOK_BACK_LENGTH as i64;
                let steps_in_cycle = last_cycle_step - first_cycle_step + 1;

                const TARGET: i64 = 1_000_000_000_000;
                let steps_left_after_cycle_start = TARGET - first_cycle_step;
                let height_at_cycle_start = tower_heights[first_cycle_step as usize - 1];
                let height_at_cycle_end = tower_heights[last_cycle_step as usize];
                let cycle_height_bump = height_at_cycle_end - height_at_cycle_start;

                let full_cycles_needed = steps_left_after_cycle_start / steps_in_cycle;
                let partial_cycle_steps = steps_left_after_cycle_start % steps_in_cycle;
                let partial_cycle_height = if partial_cycle_steps > 0 {
                    tower_heights[(first_cycle_step + partial_cycle_steps) as usize - 1]
                        - height_at_cycle_start
                } else {
                    0
                };

                let part_2 = height_at_cycle_start
                    + cycle_height_bump * full_cycles_needed
                    + partial_cycle_height;
                println!("Part 2: {part_2}");
                break;
            }
        }
    }
}
