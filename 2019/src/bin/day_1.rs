extern crate util;

fn main() {
    let inp: Vec<i64> = util::get_day_input(1)
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let part1: i64 = inp.iter().map(|x| x / 3 - 2).sum();
    println!("Part 1: {}", part1);

    let part2: i64 = inp
        .iter()
        .map(|&x| {
            let mut fuel = 0;
            let mut weight_added = x / 3 - 2;
            while weight_added > 0 {
                fuel += weight_added;
                weight_added = weight_added / 3 - 2;
            }
            fuel
        })
        .sum();
    println!("Part 2: {}", part2);
}
