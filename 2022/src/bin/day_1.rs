use itertools::Itertools;

extern crate util;

fn input() -> Vec<Vec<u64>> {
    util::get_day_input(1)
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>())
        .collect()
}

fn main() {
    let inventory = input();
    {
        let _timer = util::PerfTimer::new("Part 1");
        let part_1 = inventory
            .iter()
            .map(|ls| ls.iter().sum::<u64>())
            .max()
            .unwrap();
        println!("Part 1: {part_1}")
    }

    {
        let _timer = util::PerfTimer::new("Part 2");
        let part_2 = inventory
            .iter()
            .map(|ls| ls.iter().sum::<u64>())
            .sorted()
            .rev()
            .take(3)
            .sum::<u64>();
        println!("Part 2: {part_2}");
    }
}
