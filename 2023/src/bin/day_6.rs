#![feature(float_next_up_down)]

use itertools::Itertools;
use util::PerfTimer;

fn input_part_1() -> Vec<(u64, u64)> {
    let raw = util::get_day_input(6);
    let lines = raw.lines().collect_vec();
    assert_eq!(lines.len(), 2);
    let times: Vec<u64> = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .try_collect()
        .unwrap();
    let distances: Vec<u64> = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .try_collect()
        .unwrap();
    assert_eq!(times.len(), distances.len());
    times.into_iter().zip(distances).collect()
}

fn input_part_2() -> (u64, u64) {
    let raw = util::get_day_input(6);
    let lines = raw.lines().collect_vec();
    assert_eq!(lines.len(), 2);
    let time = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    (time, distance)
}

fn simulate(time: u64, hold_button: u64) -> u64 {
    let time = time - hold_button;
    time * hold_button
}

fn main() {
    {
        let _timer = PerfTimer::new("Part 1");
        let races = input_part_1();
        let part_1 = races
            .into_iter()
            .map(|(time, record)| {
                (0..=time)
                    .map(|x| simulate(time, x))
                    .filter(|&d| d > record)
                    .count()
            })
            .reduce(std::ops::Mul::mul)
            .unwrap();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let (time, record) = input_part_2();

        // r = (t-h)h = -h^2 + th => -h^2 + th - r = 0
        // => h = (-t ± sqrt(t^2 - 4r)) / -2
        let t = time as f64;
        let r = record as f64;
        let s1 = (-t + (t * t - 4. * r).sqrt()) / -2.;
        let s2 = (-t - (t * t - 4. * r).sqrt()) / -2.;
        let s_min = s1.min(s2).next_up().ceil() as u64;
        let s_max = s1.max(s2).next_down().floor() as u64;
        let part_2 = s_max - s_min + 1;
        println!("Part 2: {part_2}");
    }
}
