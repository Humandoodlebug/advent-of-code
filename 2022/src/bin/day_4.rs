use util::PerfTimer;

fn input() -> Vec<((usize, usize), (usize, usize))> {
    util::get_day_input(4)
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let (l_a, l_b) = l.split_once('-').unwrap();
            let (r_a, r_b) = r.split_once('-').unwrap();
            (
                (l_a.parse().unwrap(), l_b.parse().unwrap()),
                (r_a.parse().unwrap(), r_b.parse().unwrap()),
            )
        })
        .collect()
}

fn main() {
    let assignments = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = assignments
            .iter()
            .copied()
            .filter(|&((l_a, l_b), (r_a, r_b))| {
                l_a <= r_a && l_b >= r_b || r_a <= l_a && r_b >= l_b
            })
            .count();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = assignments
            .iter()
            .copied()
            .filter(|&((l_a, l_b), (r_a, r_b))| {
                l_a <= r_b && l_b >= r_a
            })
            .count();

        println!("Part 2: {part_2}");
    }
}
