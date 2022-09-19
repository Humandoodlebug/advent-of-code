use std::cmp::*;

use util::PerfTimer;

extern crate util;

fn input() -> Vec<Vec<i32>> {
    util::get_day_input(11)
        .lines()
        .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect()
}

fn main() {
    let mut grid = input();

    let mut part_1_timer = PerfTimer::new("Part 1");
    let mut part_2_timer = PerfTimer::new("Part 2");

    let x_len = grid.len();
    let y_len = grid[0].len();
    let mut flashes = 0;
    let mut part1 = false;
    let mut part2 = false;
    for step in 1.. {
        let mut to_flash = vec![];
        for (x, l) in grid.iter_mut().enumerate() {
            for (y, c) in l.iter_mut().enumerate() {
                *c += 1;
                if *c == 10 {
                    to_flash.push((x, y));
                }
            }
        }

        let mut flashed = vec![];
        while let Some((x, y)) = to_flash.pop() {
            flashes += 1;
            let i_base = max(0, x as i32 - 1) as usize;
            let j_base = max(0, y as i32 - 1) as usize;

            for (i, line) in grid[i_base..min(x_len, x + 2)].iter_mut().enumerate() {
                for (j, c) in line[j_base..min(y_len, y + 2)].iter_mut().enumerate() {
                    *c += 1;
                    if *c == 10 {
                        to_flash.push((i_base + i, j_base + j));
                    }
                }
            }
            flashed.push((x, y));
        }
        if step == 100 {
            println!("Part 1: {}", flashes);
            part1 = true;
            part_1_timer.stop();
            part_1_timer.print();
        }
        if !part2 && flashed.len() == x_len * y_len {
            println!("Part 2: {}", step);
            part2 = true;
            part_2_timer.stop();
            part_2_timer.print();
        }
        if part1 && part2 {
            break;
        }
        for (x, y) in flashed {
            grid[x][y] = 0;
        }
    }
}
