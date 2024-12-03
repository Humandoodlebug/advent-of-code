use itertools::Itertools;
use regex::RegexBuilder;
use util::PerfTimer;

fn main() {
    let input = util::get_day_input(3);

    {
        let _timer = PerfTimer::new("Part 1");
        let re = RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .dot_matches_new_line(true)
            .multi_line(false)
            .build()
            .unwrap();
        let part_1 = re
            .captures_iter(&input)
            .map(|caps| {
                let (_, [a, b]) = caps.extract();
                let a: u64 = a.parse().unwrap();
                let b: u64 = b.parse().unwrap();
                a * b
            })
            .sum::<u64>();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let re = RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))")
            .dot_matches_new_line(true)
            .multi_line(false)
            .build()
            .unwrap();
        let mut acc = 0;
        let mut enabled = true;
        for caps in re.captures_iter(&input) {
            match caps.iter().collect_tuple().unwrap() {
                (_, Some(a), Some(b), None, None) => {
                    if enabled {
                        let a: u64 = a.as_str().parse().unwrap();
                        let b: u64 = b.as_str().parse().unwrap();
                        acc += a * b;
                    }
                }
                (_, None, None, Some(_do), None) => {
                    enabled = true;
                }
                (_, None, None, None, Some(_dont)) => {
                    enabled = false;
                }
                _ => unreachable!(),
            }
        }

        println!("Part 2: {acc}");
    }
}
