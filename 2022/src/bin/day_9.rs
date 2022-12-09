use std::{collections::HashSet, cmp::Ordering};

use util::PerfTimer;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn input() -> Vec<(Direction, i64)> {
    util::get_day_input(9)
        .trim()
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            let direction = match direction {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                o => panic!("Couldn't parse {o:?} into a direction"),
            };
            let distance = distance.parse().unwrap();
            (direction, distance)
        })
        .collect()
}

fn calculate_tail_pos(head_pos: (i64, i64), mut tail_pos: (i64, i64)) -> (i64, i64) {
    if head_pos.0.abs_diff(tail_pos.0) > 1 || head_pos.1.abs_diff(tail_pos.1) > 1 {
        match tail_pos.0.cmp(&head_pos.0) {
            Ordering::Less => tail_pos.0 += 1,
            Ordering::Greater => tail_pos.0 -= 1,
            Ordering::Equal => {}
        }
        match tail_pos.1.cmp(&head_pos.1) {
            Ordering::Less => tail_pos.1 += 1,
            Ordering::Greater => tail_pos.1 -= 1,
            Ordering::Equal => {}
        }
    }
    tail_pos
}

fn simulate(instructions: &[(Direction, i64)], knot_count: usize) -> usize {
    let mut knots = vec![(0i64, 0i64); knot_count];
    let mut tail_visited = HashSet::new();
    tail_visited.insert(knots[knot_count - 1]);
    for &(direction, distance) in instructions {
        for _ in 0..distance {
            match direction {
                Direction::Up => {
                    knots[0].0 -= 1;
                }
                Direction::Right => {
                    knots[0].1 += 1;
                }
                Direction::Down => {
                    knots[0].0 += 1;
                }
                Direction::Left => {
                    knots[0].1 -= 1;
                }
            }
            let mut head_pos = knots[0];
            for tail_pos in knots.iter_mut().skip(1) {
                *tail_pos = calculate_tail_pos(head_pos, *tail_pos);
                head_pos = *tail_pos;
            }
            tail_visited.insert(knots[knot_count - 1]);
        }
    }
    tail_visited.len()
}

fn main() {
    let instructions = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = simulate(&instructions, 2);
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = simulate(&instructions, 10);
        println!("Part 2: {part_2}");
    }
}
