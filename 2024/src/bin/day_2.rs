use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<Vec<u64>> {
    let input = util::get_day_input(2);
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn check_condition(
    report: impl Iterator<Item = u64>,
    condition: impl Fn(u64, u64) -> bool,
) -> bool {
    report.tuple_windows().all(|(a, b)| condition(a, b))
}

fn main() {
    let input = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = input
            .iter()
            .filter(|report| {
                check_condition(report.iter().copied(), |a, b| a < b && b - a <= 3)
                    || check_condition(report.iter().copied(), |a, b| a > b && a - b <= 3)
            })
            .count();
        println!("Part 1: {part_1}");
    }

    {
        fn check_condition_relaxed(report: &[u64], condition: impl Fn(u64, u64) -> bool) -> bool {
            struct SkipElemIter<I: Iterator> {
                iter: I,
                index: usize,
                skip_index: usize,
            }
            impl<I: Iterator> SkipElemIter<I> {
                pub fn new(iter: I, skip_index: usize) -> Self {
                    Self {
                        iter,
                        index: 0,
                        skip_index,
                    }
                }
            }
            impl<I: Iterator> Iterator for SkipElemIter<I> {
                type Item = I::Item;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.index == self.skip_index {
                        self.index += 1;
                        self.iter.next();
                    }
                    self.index += 1;
                    self.iter.next()
                }
            }

            if check_condition(report.iter().copied(), &condition) {
                return true;
            }
            for i in 0..report.len() {
                let report = SkipElemIter::new(report.iter().copied(), i);
                if check_condition(report, &condition) {
                    return true;
                }
            }

            false
        }

        let _timer = PerfTimer::new("Part 2");
        let part_2 = input
            .iter()
            .filter(|report| {
                check_condition_relaxed(report, |a, b| a < b && b - a <= 3)
                    || check_condition_relaxed(report, |a, b| a > b && a - b <= 3)
            })
            .count();
        println!("Part 2: {part_2}");
    }
}
