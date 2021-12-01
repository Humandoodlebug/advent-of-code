#![feature(array_windows)]

fn main() {
    let depths: Vec<i32> = std::fs::read_to_string("input/day1.txt")
        .unwrap()
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let part1 = depths.array_windows().filter(|[x, y]| x < y).count();
    println!("Part 1: {}", part1);

    let part2 = depths
        .array_windows()
        .map(|[x, y, z]| x + y + z)
        .collect::<Vec<i32>>()
        .array_windows()
        .filter(|[x, y]| x < y)
        .count();

    println!("Part 2: {}", part2);
}
