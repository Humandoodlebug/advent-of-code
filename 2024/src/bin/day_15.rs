#![allow(clippy::similar_names, clippy::match_on_vec_items)]

use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Box,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell2 {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn apply_to(self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Move::Up => (x, y - 1),
            Move::Down => (x, y + 1),
            Move::Left => (x - 1, y),
            Move::Right => (x + 1, y),
        }
    }
}

fn input() -> (Vec<Vec<Cell>>, (usize, usize), Vec<Move>) {
    let mut robot_pos = None;
    let raw = util::get_day_input(15);
    let mut lines = raw.lines();
    let map = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'O' => Cell::Box,
                    '@' => {
                        robot_pos = Some((x, y));
                        Cell::Empty
                    }
                    _ => panic!("unexpected character in input map: {c:?}"),
                })
                .collect()
        })
        .collect();

    let moves = lines
        .join("")
        .chars()
        .map(|c| match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("unexpected character in move list: {c:?}"),
        })
        .collect();

    (map, robot_pos.expect("no robot found on input map"), moves)
}

fn shift_cell(map: &mut Vec<Vec<Cell>>, (x, y): (usize, usize), m: Move) -> bool {
    match map[y][x] {
        Cell::Empty => {
            // Cell is empty, so no need to clear it
            true
        }
        Cell::Wall => {
            // Cell is a wall, so we can't move it
            false
        }
        Cell::Box => {
            // Cell is a box, so we can try to move it
            let (new_x, new_y) = m.apply_to((x, y));
            if shift_cell(map, (new_x, new_y), m) {
                map[new_y][new_x] = Cell::Box;
                map[y][x] = Cell::Empty;
                true
            } else {
                false
            }
        }
    }
}

fn shift_cell_2(map: &mut Vec<Vec<Cell2>>, (x, y): (usize, usize), m: Move, dry_run: bool) -> bool {
    match map[y][x] {
        Cell2::Empty => {
            // Cell is empty, so no need to clear it
            true
        }
        Cell2::Wall => {
            // Cell is a wall, so we can't move it
            false
        }
        Cell2::BoxLeft => {
            // Cell is left side of a box, so we can try to move it
            match m {
                Move::Up | Move::Down => {
                    let (new_x, new_y) = m.apply_to((x, y));
                    let (x_right, y_right) = Move::Right.apply_to((x, y));
                    let (new_x_right, new_y_right) = m.apply_to((x_right, y_right));
                    if shift_cell_2(map, (new_x, new_y), m, true)
                        && shift_cell_2(map, (new_x_right, new_y_right), m, true)
                    {
                        if !dry_run {
                            assert!(shift_cell_2(map, (new_x, new_y), m, false));
                            assert!(shift_cell_2(map, (new_x_right, new_y_right), m, false));
                            map[new_y][new_x] = Cell2::BoxLeft;
                            map[y][x] = Cell2::Empty;
                            map[new_y_right][new_x_right] = Cell2::BoxRight;
                            map[y_right][x_right] = Cell2::Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                Move::Left => {
                    let (new_x, new_y) = m.apply_to((x, y));
                    let (x_right, y_right) = Move::Right.apply_to((x, y));
                    if shift_cell_2(map, (new_x, new_y), m, dry_run) {
                        if !dry_run {
                            map[new_y][new_x] = Cell2::BoxLeft;
                            map[y][x] = Cell2::BoxRight;
                            map[y_right][x_right] = Cell2::Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                Move::Right => {
                    let (x_right, y_right) = Move::Right.apply_to((x, y));
                    let (new_x_right, new_y_right) = m.apply_to((x_right, y_right));
                    if shift_cell_2(map, (new_x_right, new_y_right), m, dry_run) {
                        if !dry_run {
                            map[new_y_right][new_x_right] = Cell2::BoxRight;
                            map[y_right][x_right] = Cell2::BoxLeft;
                            map[y][x] = Cell2::Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
            }
        }
        Cell2::BoxRight => {
            // Just redirect to a left box move to simplify the logic
            shift_cell_2(map, Move::Left.apply_to((x, y)), m, dry_run)
        }
    }
}

fn move_robot(map: &mut Vec<Vec<Cell>>, robot_pos: &mut (usize, usize), m: Move) -> bool {
    let (new_x, new_y) = m.apply_to(*robot_pos);
    if shift_cell(map, (new_x, new_y), m) {
        *robot_pos = (new_x, new_y);
        true
    } else {
        false
    }
}

fn move_robot_2(map: &mut Vec<Vec<Cell2>>, robot_pos: &mut (usize, usize), m: Move) -> bool {
    let (new_x, new_y) = m.apply_to(*robot_pos);
    if shift_cell_2(map, (new_x, new_y), m, false) {
        *robot_pos = (new_x, new_y);
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Cell>], robot: &(usize, usize)) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            print!(
                "{}",
                if *robot == (x, y) {
                    '@'
                } else {
                    match cell {
                        Cell::Empty => '.',
                        Cell::Wall => '#',
                        Cell::Box => 'O',
                    }
                }
            );
        }
        println!();
    }
    println!();
    println!();
}

#[allow(dead_code)]
fn print_map_2(map: &[Vec<Cell2>], robot: &(usize, usize)) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            print!(
                "{}",
                if *robot == (x, y) {
                    '@'
                } else {
                    match cell {
                        Cell2::Empty => '.',
                        Cell2::Wall => '#',
                        Cell2::BoxLeft => '[',
                        Cell2::BoxRight => ']',
                    }
                }
            );
        }
        println!();
    }
    println!();
    println!();
}

fn part_2_map_from_map(
    map: &[Vec<Cell>],
    robot_pos: &(usize, usize),
) -> (Vec<Vec<Cell2>>, (usize, usize)) {
    let new_map = map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    Cell::Empty => [Cell2::Empty, Cell2::Empty],
                    Cell::Wall => [Cell2::Wall, Cell2::Wall],
                    Cell::Box => [Cell2::BoxLeft, Cell2::BoxRight],
                })
                .collect()
        })
        .collect();

    let new_robot_pos = (robot_pos.0 * 2, robot_pos.1);

    (new_map, new_robot_pos)
}

fn main() {
    let (map, robot_pos, moves) = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut map = map.clone();
        let mut robot_pos = robot_pos;
        for &m in &moves {
            // println!("Move: {m:?}");
            move_robot(&mut map, &mut robot_pos, m);
            // print_map(&map, &robot_pos);
        }

        let part_1: usize = map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, cell)| {
                        if *cell == Cell::Box {
                            Some(y * 100 + x)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let (mut map, mut robot_pos) = part_2_map_from_map(&map, &robot_pos);
        // print_map_2(&map, &robot_pos);
        for &m in &moves {
            // println!("Move: {m:?}");
            move_robot_2(&mut map, &mut robot_pos, m);
            // print_map_2(&map, &robot_pos);
        }

        let part_2: usize = map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, cell)| {
                        if *cell == Cell2::BoxLeft {
                            Some(y * 100 + x)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum();

        println!("Part 2: {part_2}");
    }
}
