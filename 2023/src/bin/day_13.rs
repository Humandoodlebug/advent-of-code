use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<Vec<Vec<bool>>> {
    let mut patterns = Vec::new();
    let mut pattern: Vec<Vec<bool>> = Vec::new();
    for line in util::get_day_input(13).lines() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().map(|c| c == '#').collect());
        }
    }
    patterns.push(pattern);
    patterns
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Symmetry {
    AfterRow(usize),
    AfterCol(usize),
}

fn find_symmetry(pattern: &[Vec<bool>]) -> Vec<Symmetry> {
    let mut lines_of_symmetry = Vec::new();
    'a: for i in 0..pattern.len() - 1 {
        let distance = (i + 1).min(pattern.len() - i - 1);
        for j in 0..distance {
            let r1 = &pattern[i - j];
            let r2 = &pattern[i + j + 1];
            if r1 != r2 {
                continue 'a;
            }
        }
        lines_of_symmetry.push(Symmetry::AfterRow(i))
    }
    'a: for i in 0..pattern[0].len() - 1 {
        let distance = (i + 1).min(pattern[0].len() - i - 1);
        for j in 0..distance {
            let r1 = &pattern.iter().map(|r| r[i - j]).collect_vec();
            let r2 = &pattern.iter().map(|r| r[i + j + 1]).collect_vec();
            if r1 != r2 {
                continue 'a;
            }
        }
        lines_of_symmetry.push(Symmetry::AfterCol(i));
    }
    lines_of_symmetry
}

fn main() {
    let patterns = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let symmetries = patterns.iter().map(|p| find_symmetry(p)[0]).collect_vec();
        let part_1: usize = symmetries
            .iter()
            .map(|s| match s {
                Symmetry::AfterRow(i) => 100 * (i + 1),
                Symmetry::AfterCol(i) => i + 1,
            })
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut patterns = patterns;
        let symmetries = patterns
            .iter_mut()
            .map(|pattern| {
                let symmetry1 = find_symmetry(pattern)[0];
                for i in 0..pattern.len() {
                    for j in 0..pattern[0].len() {
                        pattern[i][j] = !pattern[i][j];

                        for symmetry in find_symmetry(pattern) {
                            if symmetry != symmetry1 {
                                return symmetry;
                            }
                        }

                        pattern[i][j] = !pattern[i][j];
                    }
                }
                panic!();
            })
            .collect_vec();
        let part_2: usize = symmetries
            .iter()
            .map(|s| match s {
                Symmetry::AfterRow(i) => 100 * (i + 1),
                Symmetry::AfterCol(i) => i + 1,
            })
            .sum();
        println!("Part 2: {part_2}");
    }
}
