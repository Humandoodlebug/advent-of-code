#![feature(in_band_lifetimes)]

extern crate util;

pub fn input() -> Vec<Vec<char>> {
    util::get_day_input(3)
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

fn common_bits(
    bits: impl IntoIterator<Item = &'a char>,
    selector: impl Fn((i32, i32)) -> char,
) -> char {
    let mut counts = (0, 0);
    for &b in bits {
        match b {
            '0' => counts.0 += 1,
            '1' => counts.1 += 1,
            _ => panic!("AHHHHHHH"),
        }
    }
    selector(counts)
}

fn bin_to_dec(bin: &[char]) -> i64 {
    bin.iter().fold(0, |acc, c| match c {
        '0' => 0,
        '1' => 1,
        _ => panic!("Not binary?"),
    } + acc * 2)
}

fn part2(inp: &[Vec<char>], selector: impl Fn((i32, i32)) -> char) -> i64 {
    let mut remaining: Vec<&Vec<char>> = inp.iter().collect();
    let mut i = 0;
    while remaining.len() > 1 {
        let column: Vec<char> = remaining.iter().map(|x| x[i]).collect();
        let keeper = common_bits(&column, &selector);
        remaining = remaining.into_iter().filter(|x| x[i] == keeper).collect();
        i += 1;
    }
    bin_to_dec(remaining[0])
}

pub fn main() {
    let inp = input();
    let (gamma, epsilon) = inp
        .iter()
        .fold(vec![(0, 0); inp[0].len()], |mut counts, num| {
            for i in 0..num.len() {
                if num[i] == '0' {
                    counts[i].0 += 1;
                } else {
                    counts[i].1 += 1;
                }
            }
            counts
        })
        .into_iter()
        .map(|x| if x.0 > x.1 { (0, 1) } else { (1, 0) })
        .fold((0i64, 0i64), |(acc_a, acc_b), (x_a, x_b)| {
            (acc_a * 2 + x_a, acc_b * 2 + x_b)
        });
    println!("Part 1: {}", gamma * epsilon);

    let oxygen = part2(&inp, |x| if x.0 > x.1 { '0' } else { '1' });
    let co2 = part2(&inp, |x| if x.1 < x.0 { '1' } else { '0' });
    println!("Part 2: {}", oxygen * co2)
}
