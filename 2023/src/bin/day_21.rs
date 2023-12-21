use std::collections::HashSet;

use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

fn input() -> (Vec<Vec<Tile>>, (usize, usize)) {
    let mut start = None;
    let map = util::get_day_input(21)
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    'S' => {
                        start = Some((row, col));
                        Tile::Plot
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (map, start.unwrap())
}

fn search_steps(
    map: &[Vec<Tile>],
    start: (usize, usize),
    target_steps: u64,
) -> Vec<(usize, usize)> {
    let mut evens = HashSet::new();
    let mut odds = HashSet::new();
    let mut states = HashSet::from_iter(std::iter::once(start));
    for step in 0..target_steps {
        let mut new_states = HashSet::new();
        let processed = if step % 2 == 0 { &mut evens } else { &mut odds };
        for (row, col) in states {
            if !processed.insert((row, col)) {
                continue;
            }
            let mut moves = Vec::new();
            if row < map.len() - 1 {
                moves.push((row + 1, col));
            }
            if col < map[0].len() - 1 {
                moves.push((row, col + 1));
            }
            if row > 0 {
                moves.push((row - 1, col));
            }
            if col > 0 {
                moves.push((row, col - 1));
            }
            for (new_row, new_col) in moves {
                if map[new_row][new_col] == Tile::Plot && !processed.contains(&(new_row, new_col)) {
                    new_states.insert((new_row, new_col));
                }
            }
        }
        states = new_states;
    }
    let mut processed = if target_steps % 2 == 0 { evens } else { odds };
    for s in states {
        processed.insert(s);
    }
    processed.into_iter().collect()
}

fn main() {
    let (map, start) = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = search_steps(&map, start, 64).len();
        println!("Part 1: {part_1}");
    }
    {
        // ((26501365 + 0.5) / 131) * 2 = 404601 (diamonds wide/tall)
        // Total odd/even exterior diamonds = sum(1 ..= (404601 - 1) / 2) * 2
        // Total odd/even interior diamonds (minus odd off-balance extras) = (404601 - 1)^2 / 4
        // Total odd extra off-balance interior diamonds = 404601

        let _timer = PerfTimer::new("Part 2");
        let total_plots =
            (search_steps(&map, start, 150).len() + search_steps(&map, start, 151).len()) as u64;
        let diamond_interior_odd = search_steps(&map, start, 65).len() as u64;
        let diamond_interior_even = search_steps(&map, start, 64).len() as u64;
        let diamond_interior_total = diamond_interior_odd + diamond_interior_even;
        let diamond_exterior_total = total_plots - diamond_interior_total;
        let odd_or_even_exterior_diamond_count = {
            let s = (404601 - 1) / 2;
            let sum_s = (s * (s + 1)) / 2;
            sum_s * 2
        };
        let odd_or_even_interior_diamonds = (404601 - 1) * (404601 - 1) / 4;
        let odd_extra_interior_diamonds = 404601;
        let part_2 = odd_or_even_exterior_diamond_count * diamond_exterior_total
            + odd_or_even_interior_diamonds * diamond_interior_total
            + odd_extra_interior_diamonds * diamond_interior_odd;

        println!("Part 2: {part_2}");
    }
}
