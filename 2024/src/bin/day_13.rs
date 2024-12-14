use itertools::Itertools;
use regex::Regex;
use util::PerfTimer;

type Point = (usize, usize);

#[derive(Clone, Copy, Debug)]
struct Machine {
    a_action: Point,
    b_action: Point,
    prize_location: Point,
}

fn input() -> Vec<Machine> {
    let button_regex = Regex::new(r"^Button (A|B): X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let raw = util::get_day_input(13);
    raw.trim()
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut chunks| {
            let (_, [_, a_x, a_y]) = button_regex
                .captures(chunks.next().unwrap())
                .unwrap()
                .extract();

            let (_, [_, b_x, b_y]) = button_regex
                .captures(chunks.next().unwrap())
                .unwrap()
                .extract();

            let (_, [prize_x, prize_y]) = prize_regex
                .captures(chunks.next().unwrap())
                .unwrap()
                .extract();

            Machine {
                a_action: (a_x.parse().unwrap(), a_y.parse().unwrap()),
                b_action: (b_x.parse().unwrap(), b_y.parse().unwrap()),
                prize_location: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
            }
        })
        .collect()
}

fn main() {
    let machines = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: usize = machines
            .iter()
            .filter_map(|machine| {
                let Machine {
                    a_action: (a_x, a_y),
                    b_action: (b_x, b_y),
                    prize_location: (prize_x, prize_y),
                } = *machine;

                let mut min_cost: Option<usize> = None;

                for a_mul in 0.. {
                    if a_mul * a_x > prize_x || a_mul * a_y > prize_y {
                        break;
                    }

                    let rem_x = prize_x - a_mul * a_x;
                    let rem_y = prize_y - a_mul * a_y;
                    if rem_x % b_x == 0 && rem_y == (rem_x / b_x) * b_y {
                        let b_mul = rem_x / b_x;
                        let cost = a_mul * 3 + b_mul;
                        min_cost = Some(min_cost.map_or(cost, |min_cost| min_cost.min(cost)));
                    }
                }

                min_cost
            })
            .sum();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let part_2: usize = machines
            .iter()
            .filter_map(|machine| {
                #[allow(clippy::cast_precision_loss)]
                fn f(v: usize) -> f64 {
                    v as f64
                }

                let Machine {
                    a_action: (a_x, a_y),
                    b_action: (b_x, b_y),
                    prize_location: (prize_x, prize_y),
                } = *machine;

                let prize_x = prize_x + 10_000_000_000_000;
                let prize_y = prize_y + 10_000_000_000_000;

                // I did some maths here - if you model the machine with linear equations, you can solve for the
                // multipliers. A whiteboard was involved... just trust it works xD
                let a_mul =
                    (f(prize_y) * f(b_x) - f(prize_x) * f(b_y)) / (f(a_y * b_x) - f(a_x * b_y));

                let b_mul = (f(prize_x) - f(a_x) * a_mul) / f(b_x);

                if a_mul < 0. || b_mul < 0. || a_mul.fract() != 0. || b_mul.fract() != 0. {
                    return None;
                }

                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Some(a_mul as usize * 3 + b_mul as usize)
            })
            .sum();

        println!("Part 2: {part_2}");
    }
}
