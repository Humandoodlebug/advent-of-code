use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<Vec<char>> {
    util::get_day_input(8)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().copied().enumerate() {
            if c != '.' {
                assert!(c.is_ascii_alphanumeric());
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()));
            }
        }
    }
    antennas
}

fn main() {
    let grid = input();

    let x_len = i32::try_from(grid[0].len()).unwrap();
    let y_len = i32::try_from(grid.len()).unwrap();
    let frequency_to_antennas = antennas(&grid);

    {
        let _timer = PerfTimer::new("Part 1");

        let mut antinodes = HashSet::new();
        for antennas in frequency_to_antennas.values() {
            for ((a_x, a_y), (b_x, b_y)) in antennas.iter().tuple_combinations() {
                let d_x = b_x - a_x;
                let d_y = b_y - a_y;

                let antinode_1_x = a_x - d_x;
                let antinode_1_y = a_y - d_y;
                let antinode_2_x = b_x + d_x;
                let antinode_2_y = b_y + d_y;

                if (0..x_len).contains(&antinode_1_x) && (0..y_len).contains(&antinode_1_y) {
                    antinodes.insert((antinode_1_x, antinode_1_y));
                }

                if (0..x_len).contains(&antinode_2_x) && (0..y_len).contains(&antinode_2_y) {
                    antinodes.insert((antinode_2_x, antinode_2_y));
                }
            }
        }

        let part_1 = antinodes.len();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");

        let mut antinodes = HashSet::new();
        for antennas in frequency_to_antennas.values() {
            for (&(a_x, a_y), &(b_x, b_y)) in antennas.iter().tuple_combinations() {
                let d_x = b_x - a_x;
                let d_y = b_y - a_y;

                let mut antinode_1_x = a_x;
                let mut antinode_1_y = a_y;
                while (0..x_len).contains(&antinode_1_x) && (0..y_len).contains(&antinode_1_y) {
                    antinodes.insert((antinode_1_x, antinode_1_y));
                    antinode_1_x -= d_x;
                    antinode_1_y -= d_y;
                }

                let mut antinode_2_x = b_x;
                let mut antinode_2_y = b_y;
                while (0..x_len).contains(&antinode_2_x) && (0..y_len).contains(&antinode_2_y) {
                    antinodes.insert((antinode_2_x, antinode_2_y));
                    antinode_2_x += d_x;
                    antinode_2_y += d_y;
                }
            }
        }

        let part_2 = antinodes.len();
        println!("Part 2: {part_2}");
    }
}
