use std::collections::HashMap;

use itertools::Itertools;

fn input() -> Vec<u64> {
    util::get_day_input(11)
        .trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn count_digits(x: u64) -> u32 {
    let mut count: u32 = 1;
    while (x / 10u64.pow(count)) > 0 {
        count += 1;
    }
    count
}

fn split_stone(stone: u64) -> Option<(u64, u64)> {
    let digit_count = count_digits(stone);
    if digit_count % 2 == 0 {
        let left = stone / 10u64.pow(digit_count / 2);
        let right = stone % 10u64.pow(digit_count / 2);
        Some((left, right))
    } else {
        None
    }
}

enum StoneResult {
    One(u64),
    Two(u64, u64),
}

fn blink_stone(stone: u64) -> StoneResult {
    if stone == 0 {
        StoneResult::One(1)
    } else if let Some((left, right)) = split_stone(stone) {
        StoneResult::Two(left, right)
    } else {
        StoneResult::One(stone * 2024)
    }
}

// Initial part 1 implementation
#[allow(dead_code)]
fn count_stones_after_blinks(stone: u64, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }

    match blink_stone(stone) {
        StoneResult::One(stone) => count_stones_after_blinks(stone, blinks - 1),
        StoneResult::Two(left, right) => {
            count_stones_after_blinks(left, blinks - 1)
                + count_stones_after_blinks(right, blinks - 1)
        }
    }
}

fn count_stones_after_blinks_dedup(stones: &[u64], blinks: usize) -> usize {
    let mut stones: HashMap<u64, usize> = stones.iter().copied().counts();

    for _ in 0..blinks {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            match blink_stone(stone) {
                StoneResult::One(stone) => {
                    *new_stones.entry(stone).or_insert(0) += count;
                }
                StoneResult::Two(left, right) => {
                    *new_stones.entry(left).or_insert(0) += count;
                    *new_stones.entry(right).or_insert(0) += count;
                }
            }
        }
        stones = new_stones;
    }

    stones.values().sum()
}

fn main() {
    let initial_stones = input();
    {
        let _timer = util::PerfTimer::new("Part 1");

        // // Initial part 1 implementation
        // let part_1: usize = initial_stones
        //     .iter()
        //     .map(|&s| count_stones_after_blinks(s, 25))
        //     .sum();
        // println!("Part 1: {part_1}");

        let part_1 = count_stones_after_blinks_dedup(&initial_stones, 25);
        println!("Part 1: {part_1}");
    }

    {
        let _timer = util::PerfTimer::new("Part 2");

        let part_2 = count_stones_after_blinks_dedup(&initial_stones, 75);
        println!("Part 2: {part_2}");
    }
}
