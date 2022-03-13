use std::collections::HashMap;

use lazy_static::lazy_static;
use priority_queue::PriorityQueue;
use util::get_day_input;
use Piece::*;
use Tile::*;

extern crate util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Piece {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Empty,
    Has(Piece),
}

type Grid<const H: usize> = [[Tile; H]; 13];

type State<const H: usize> = (Grid<H>, usize);

type Point = (usize, usize);

lazy_static! {
    static ref COST: HashMap<Piece, usize> = HashMap::from([(A, 1), (B, 10), (C, 100), (D, 1000)]);
}

lazy_static! {
    static ref COLUMN: HashMap<Piece, usize> = HashMap::from([(A, 3), (B, 5), (C, 7), (D, 9)]);
}

fn input() -> (Grid<5>, Grid<7>) {
    let raw = get_day_input(23);
    let lines_1 = raw.lines().take_while(|&l| !l.is_empty());
    let lines_2 = raw.lines().skip_while(|&l| !l.is_empty()).skip(1);

    let mut grid_1 = [[Wall; 5]; 13];
    for (y, line) in lines_1.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid_1[x][y] = match c {
                '.' => Empty,
                '#' => Wall,
                'A' => Has(A),
                'B' => Has(B),
                'C' => Has(C),
                'D' => Has(D),
                ' ' => Wall,
                _ => panic!(),
            }
        }
    }

    let mut grid_2 = [[Wall; 7]; 13];
    for (y, line) in lines_2.enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid_2[x][y] = match c {
                '.' => Empty,
                '#' => Wall,
                'A' => Has(A),
                'B' => Has(B),
                'C' => Has(C),
                'D' => Has(D),
                ' ' => Wall,
                _ => panic!(),
            }
        }
    }
    (grid_1, grid_2)
}

fn swap<const H: usize>(
    (grid, cost): &State<H>,
    (x_1, y_1): Point,
    (x_2, y_2): Point,
    price: usize,
) -> State<H> {
    let mut new_grid = *grid;
    new_grid[x_1][y_1] = grid[x_2][y_2];
    new_grid[x_2][y_2] = grid[x_1][y_1];

    let new_cost = cost + (x_1.abs_diff(x_2) + y_1.abs_diff(y_2)) * price;
    (new_grid, new_cost)
}

fn gen_moves<const H: usize>((grid, cost): State<H>) -> Vec<State<H>> {
    let mut states = vec![];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if let Has(ref piece) = grid[x][y] {
                if x == COLUMN[piece] && (y + 1..H - 1).all(|y| grid[x][y] == Has(*piece)) {
                    continue;
                }
                if y >= 2 && grid[x][y - 1] == Empty {
                    let mut i = x - 1;
                    while grid[i][1] == Empty {
                        if grid[i][2] == Wall {
                            states.push(swap(&(grid, cost), (x, y), (i, 1), COST[piece]));
                        }
                        i -= 1;
                    }
                    let mut i = x + 1;
                    while grid[i][1] == Empty {
                        if grid[i][2] == Wall {
                            states.push(swap(&(grid, cost), (x, y), (i, 1), COST[piece]));
                        }
                        i += 1;
                    }
                } else if y == 1
                    && grid[COLUMN[piece]][2] == Empty
                    && (3..H - 1)
                        .skip_while(|&y| grid[COLUMN[piece]][y] == Empty)
                        .all(|y| grid[COLUMN[piece]][y] == Has(*piece))
                {
                    let target_y = (2..)
                        .take_while(|&y| grid[COLUMN[piece]][y] == Empty)
                        .last()
                        .unwrap();
                    let (start, end) = if COLUMN[piece] < x {
                        (COLUMN[piece], x - 1)
                    } else {
                        (x + 1, COLUMN[piece])
                    };
                    if (start..=end).all(|x| grid[x][1] == Empty) {
                        states.push(swap(
                            &(grid, cost),
                            (x, y),
                            (COLUMN[piece], target_y),
                            COST[piece],
                        ));
                    }
                }
            }
        }
    }
    states
}

fn is_done<const H: usize>(grid: &Grid<H>) -> bool {
    // for (&piece, &x) in COLUMN.iter() {
    //     if !gr
    // }
    COLUMN
        .iter()
        .all(|(&piece, &x)| (2..H - 1).all(|y| grid[x][y] == Has(piece)))
}

fn cost_lower_bound<const H: usize>((grid, cost): &State<H>) -> usize {
    let mut lower_bound = 0usize;
    for (x, line) in grid.iter().enumerate() {
        for (y, &c) in line.iter().enumerate() {
            if let Has(ref piece) = c {
                let mut bound_x = 0;
                if x == COLUMN[piece] {
                    if !(y + 1..H - 1).all(|y| grid[x][y] == Has(*piece)) {
                        bound_x += y * 2;
                    }
                } else {
                    bound_x += (y - 1) + x.abs_diff(COLUMN[piece]) + 1;
                }
                lower_bound += bound_x * COST[piece];
            }
        }
    }
    for (&piece, &x) in COLUMN.iter() {
        for y in 3..H - 1 {
            if grid[x][y] != Has(piece) {
                lower_bound += y - 2
            }
        }
    }
    lower_bound + cost
}

fn play<const H: usize>(grid: Grid<H>) -> usize {
    let mut pq = PriorityQueue::new();
    let initial_state = (grid, 0);
    // print_grid(&grid);
    let initial_lower_bound_estimate = cost_lower_bound(&initial_state);
    pq.push(initial_state, -(initial_lower_bound_estimate as i64));
    // let mut i = 0;
    loop {
        // i += 1;
        let (current_state, _) = pq.pop().unwrap();
        // if i % 100 == 0 {
        //     println!("Processing state: ");
        //     print_grid(&current_state.0);
        //     println!("Cost: {}", current_state.1);
        //     println!("Estimated cost: {}", cost_lower_bound(&current_state));
        //     print!("");
        // }
        let next_states = gen_moves(current_state);
        for state in next_states {
            if is_done(&state.0) {
                return state.1;
            }
            let lower_bound_estimate = cost_lower_bound(&state);
            pq.push(state, -(lower_bound_estimate as i64));
        }
    }
}

fn main() {
    let (grid_1, grid_2) = input();
    let part_1 = play(grid_1);
    println!("Part 1: {}", part_1);

    let part_2 = play(grid_2);
    println!("Part 2: {}", part_2);
}
