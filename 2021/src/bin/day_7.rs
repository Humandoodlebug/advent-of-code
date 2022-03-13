extern crate util;

fn main() {
    let mut inp: Vec<i32> = util::get_day_input(7)
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    inp.sort_unstable();
    let p1_index = inp.len() / 2;
    let p1_point = inp[p1_index];
    let part1: u32 = inp.iter().map(|x| x.abs_diff(p1_point)).sum();

    println!("Part 1: {}", part1);

    let calc = |x| x * (x + 1) / 2;

    let part2: u64 = (0..1000)
        .map(|p| inp.iter().map(|x| calc(x.abs_diff(p) as u64)).sum())
        .min()
        .unwrap();
    println!("Part 2: {}", part2);
}
