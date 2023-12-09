use itertools::{self, Itertools};

fn input() -> Vec<Vec<i64>> {
    util::get_day_input(9)
        .lines()
        .map(|line| line.split_whitespace().map(str::parse).try_collect())
        .try_collect()
        .unwrap()
}

fn main() {
    let report = input();
    let _timer = util::PerfTimer::new("Both parts");
    let (part_1, part_2) = report
        .iter()
        .map(|data| {
            let mut order_diffs = Vec::new();
            loop {
                let current_diff = order_diffs.last().unwrap_or(data);
                let new_diff: Vec<i64> = current_diff
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
                if new_diff.iter().all(|i| *i == 0) {
                    break;
                }
                order_diffs.push(new_diff);
            }

            // part 1
            let diff_end: i64 = order_diffs.iter().rev().map(|d| d.last().unwrap()).sum();
            let new_last = *data.last().unwrap() + diff_end;

            // part 2
            let diff_start: i64 = order_diffs.iter().rev().fold(0, |s, d| d[0] - s);
            let new_first = data[0] - diff_start;
            (new_first, new_last)
        })
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        .unwrap();
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
