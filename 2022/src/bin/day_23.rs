use std::collections::{HashMap, HashSet};

use util::PerfTimer;

type Point = (i64, i64);

fn input() -> HashSet<Point> {
    let raw = util::get_day_input(23);
    let mut elf_positions = HashSet::new();
    for (row, line) in raw.lines().enumerate() {
        for (column, cell) in line.chars().enumerate() {
            match cell {
                '.' => {}
                '#' => {
                    elf_positions.insert((row as i64, column as i64));
                }
                c => panic!("unexpected character {c:?} in input at ({row},{column})"),
            }
        }
    }
    elf_positions
}

fn propose_north(elf_positions: &HashSet<Point>, (elf_row, elf_column): Point) -> Option<Point> {
    for i in -1..=1 {
        if elf_positions.contains(&(elf_row - 1, elf_column + i)) {
            return None;
        }
    }
    Some((elf_row - 1, elf_column))
}

fn propose_south(elf_positions: &HashSet<Point>, (elf_row, elf_column): Point) -> Option<Point> {
    for i in -1..=1 {
        if elf_positions.contains(&(elf_row + 1, elf_column + i)) {
            return None;
        }
    }
    Some((elf_row + 1, elf_column))
}

fn propose_west(elf_positions: &HashSet<Point>, (elf_row, elf_column): Point) -> Option<Point> {
    for i in -1..=1 {
        if elf_positions.contains(&(elf_row + i, elf_column - 1)) {
            return None;
        }
    }
    Some((elf_row, elf_column - 1))
}

fn propose_east(elf_positions: &HashSet<Point>, (elf_row, elf_column): Point) -> Option<Point> {
    for i in -1..=1 {
        if elf_positions.contains(&(elf_row + i, elf_column + 1)) {
            return None;
        }
    }
    Some((elf_row, elf_column + 1))
}

fn propose(
    elf_positions: &HashSet<Point>,
    (elf_row, elf_column): Point,
    turn_offset: usize,
) -> Option<Point> {
    let mut elves_in_proximity = false;
    'row: for row in elf_row - 1..=elf_row + 1 {
        for column in elf_column - 1..=elf_column + 1 {
            if (row, column) == (elf_row, elf_column) {
                continue;
            }
            if elf_positions.contains(&(row, column)) {
                elves_in_proximity = true;
                break 'row;
            }
        }
    }

    if !elves_in_proximity {
        return None;
    }

    let proposals = [propose_north, propose_south, propose_west, propose_east];
    for i in 0..4 {
        let index = (i + turn_offset) % 4;
        let proposal = proposals[index];
        if let Some(result) = proposal(elf_positions, (elf_row, elf_column)) {
            return Some(result);
        }
    }

    None
}

fn move_elves(elf_positions: &mut HashSet<Point>, turn_offset: usize) -> bool {
    let mut proposals = HashMap::new();
    for elf_position in elf_positions.iter().copied() {
        if let Some(proposal) = propose(elf_positions, elf_position, turn_offset) {
            if proposals.remove(&proposal).is_none() {
                proposals.insert(proposal, elf_position);
            }
        }
    }

    if proposals.is_empty() {
        return false;
    }

    for (proposal, elf_position) in proposals {
        assert!(elf_positions.remove(&elf_position));
        assert!(elf_positions.insert(proposal));
    }

    true
}

// fn print_elves(elf_positions: &HashSet<Point>) {
//     let min_row = elf_positions
//         .iter()
//         .copied()
//         .map(|(row, _)| row)
//         .min()
//         .unwrap();
//     let max_row = elf_positions
//         .iter()
//         .copied()
//         .map(|(row, _)| row)
//         .max()
//         .unwrap();
//     let min_col = elf_positions
//         .iter()
//         .copied()
//         .map(|(_, col)| col)
//         .min()
//         .unwrap();
//     let max_col = elf_positions
//         .iter()
//         .copied()
//         .map(|(_, col)| col)
//         .max()
//         .unwrap();

//     for row in min_row..=max_row {
//         for col in min_col..=max_col {
//             if elf_positions.contains(&(row, col)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//     println!();
//     println!();
// }

fn main() {
    let input = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut elf_positions = input.clone();
        // print_elves(&elf_positions);
        for i in 0..10 {
            move_elves(&mut elf_positions, i);
            // println!("Round {}:", i + 1);
            // print_elves(&elf_positions);
        }

        let min_row = elf_positions
            .iter()
            .copied()
            .map(|(row, _)| row)
            .min()
            .unwrap();
        let max_row = elf_positions
            .iter()
            .copied()
            .map(|(row, _)| row)
            .max()
            .unwrap();
        let min_col = elf_positions
            .iter()
            .copied()
            .map(|(_, col)| col)
            .min()
            .unwrap();
        let max_col = elf_positions
            .iter()
            .copied()
            .map(|(_, col)| col)
            .max()
            .unwrap();

        let total_area = (max_row - min_row + 1) * (max_col - min_col + 1);
        let part_1 = total_area - elf_positions.len() as i64;

        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut elf_positions = input.clone();
        for i in 0.. {
            if !move_elves(&mut elf_positions, i) {
                println!("Part 2: {}", i + 1);
                break;
            }
        }
    }
}
