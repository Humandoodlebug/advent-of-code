use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<char> {
    util::get_day_input(6).chars().collect()
}

fn find_marker_index(signal: &[char], size: usize) -> usize {
    signal
        .windows(size)
        .find_position(|x| x.iter().all_unique())
        .unwrap()
        .0
        + size
}

fn main() {
    let signal = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = find_marker_index(&signal, 4);
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = find_marker_index(&signal, 14);
        println!("Part 2: {part_2}");
    }
}
