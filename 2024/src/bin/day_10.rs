use std::collections::HashSet;

use util::PerfTimer;

fn input() -> Vec<Vec<u32>> {
    util::get_day_input(10)
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_reachable_peaks(map: &[Vec<u32>], x: usize, y: usize) -> usize {
    fn inner(map: &[Vec<u32>], x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> usize {
        let current_height = map[y][x];
        if visited.contains(&(x, y)) {
            return 0;
        }
        visited.insert((x, y));
        if current_height == 9 {
            return 1;
        }
        let mut peaks = 0;
        if x > 0 && map[y][x - 1] == current_height + 1 {
            peaks += inner(map, x - 1, y, visited);
        }
        if x < map[y].len() - 1 && map[y][x + 1] == current_height + 1 {
            peaks += inner(map, x + 1, y, visited);
        }
        if y > 0 && map[y - 1][x] == current_height + 1 {
            peaks += inner(map, x, y - 1, visited);
        }
        if y < map.len() - 1 && map[y + 1][x] == current_height + 1 {
            peaks += inner(map, x, y + 1, visited);
        }

        peaks
    }

    let mut visited = HashSet::new();
    inner(map, x, y, &mut visited)
}

fn count_trails(map: &[Vec<u32>], x: usize, y: usize) -> usize {
    let current_height = map[y][x];
    if current_height == 9 {
        return 1;
    }
    let mut paths = 0;
    if x > 0 && map[y][x - 1] == current_height + 1 {
        paths += count_trails(map, x - 1, y);
    }
    if x < map[y].len() - 1 && map[y][x + 1] == current_height + 1 {
        paths += count_trails(map, x + 1, y);
    }
    if y > 0 && map[y - 1][x] == current_height + 1 {
        paths += count_trails(map, x, y - 1);
    }
    if y < map.len() - 1 && map[y + 1][x] == current_height + 1 {
        paths += count_trails(map, x, y + 1);
    }

    paths
}

fn main() {
    let map = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut trailhead_ratings_sum = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    trailhead_ratings_sum += count_reachable_peaks(&map, x, y);
                }
            }
        }

        println!("Part 1: {trailhead_ratings_sum}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut trail_count = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    trail_count += count_trails(&map, x, y);
                }
            }
        }
        println!("Part 2: {trail_count}");
    }
}
