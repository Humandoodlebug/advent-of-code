extern crate util;

fn main() {
    let input = util::get_day_input(10);

    let mut part1 = 0i128;

    for line in input.lines() {
        let mut remaining = vec![];
        for c in line.chars() {
            match c {
                '(' => remaining.push(')'),
                '[' => remaining.push(']'),
                '{' => remaining.push('}'),
                '<' => remaining.push('>'),
                c => if c != remaining.pop().unwrap_or('\0') {
                    part1 += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        c => panic!("Unrecognised closing bracket {:?}", c),
                    };
                }
            }
        }
    }
    println!("Part 1: {}", part1);

    let mut scores = vec![];

    for line in input.lines() {
        let mut remaining = vec![];
        for c in line.chars() {
            match c {
                '(' => remaining.push(')'),
                '[' => remaining.push(']'),
                '{' => remaining.push('}'),
                '<' => remaining.push('>'),
                c => if c != remaining.pop().unwrap_or('\0') {
                    remaining.clear();
                    break;
                }
            }
        }
        if remaining.is_empty() {
            continue;
        }

        let score = remaining.into_iter().rev().map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            c => panic!("Unrecongnised closing brace {:?}", c),
        }).fold(0i128, |s, x| s * 5 + x as i128);
        scores.push(score)
    }
    scores.sort_unstable();
    let part2 = scores[scores.len() / 2];
    println!("Part 2: {}", part2);
}
