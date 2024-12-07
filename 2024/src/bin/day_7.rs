use util::PerfTimer;

fn input() -> Vec<(u64, Vec<u64>)> {
    let raw = util::get_day_input(7);
    raw.lines()
        .map(|line| {
            let (test_value, rem) = line.split_once(": ").unwrap();
            let test_value = test_value.parse().unwrap();
            let args = rem.split(' ').map(|x| x.parse().unwrap()).collect();
            (test_value, args)
        })
        .collect()
}

fn main() {
    let equations = input();

    {
        fn try_eval(test_value: u64, acc: u64, args: &[u64]) -> bool {
            if args.is_empty() {
                acc == test_value
            } else if acc > test_value {
                false
            } else {
                try_eval(test_value, acc * args[0], &args[1..])
                    || try_eval(test_value, acc + args[0], &args[1..])
            }
        }

        let _timer = PerfTimer::new("Part 1");

        let part_1 = equations
            .iter()
            .filter_map(|(test_value, args)| {
                if try_eval(*test_value, args[0], &args[1..]) {
                    Some(*test_value)
                } else {
                    None
                }
            })
            .sum::<u64>();

        println!("Part 1: {part_1}");
    }

    {
        fn concat(a: u64, b: u64) -> u64 {
            let mut mul = 1;
            while mul <= b {
                mul *= 10;
            }
            a * mul + b
        }

        fn try_eval(test_value: u64, acc: u64, args: &[u64]) -> bool {
            if args.is_empty() {
                acc == test_value
            } else if acc > test_value {
                false
            } else {
                try_eval(test_value, acc * args[0], &args[1..])
                    || try_eval(test_value, acc + args[0], &args[1..])
                    || try_eval(test_value, concat(acc, args[0]), &args[1..])
            }
        }

        let _timer = PerfTimer::new("Part 2");

        let part_2 = equations
            .iter()
            .filter_map(|(test_value, args)| {
                if try_eval(*test_value, args[0], &args[1..]) {
                    Some(*test_value)
                } else {
                    None
                }
            })
            .sum::<u64>();

        println!("Part 2: {part_2}");
    }
}
