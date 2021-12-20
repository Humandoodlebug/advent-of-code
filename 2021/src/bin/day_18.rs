extern crate util;
use itertools::Itertools;
use Number::*;

#[derive(PartialEq, Eq, Clone)]
enum Number {
    Literal(i32),
    Pair(Box<Number>, Box<Number>),
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(x) => x.fmt(f),
            Self::Pair(a, b) => f.write_fmt(format_args!("[{:?},{:?}]", a, b)),
        }
    }
}

fn parse_number(inp: &[char]) -> (&[char], Number) {
    if inp[0] == '[' {
        let (inp, left) = parse_number(&inp[1..]);
        assert_eq!(inp[0], ',');
        let (inp, right) = parse_number(&inp[1..]);
        assert_eq!(inp[0], ']');
        (&inp[1..], Pair(Box::new(left), Box::new(right)))
    } else {
        let num: String = inp
            .iter()
            .take_while(|&&c| ('0'..='9').contains(&c))
            .collect();
        let inp = &inp[num.len()..];
        (inp, Literal(num.parse().unwrap()))
    }
}

fn input() -> Vec<Number> {
    let raw = util::get_day_input(18);
    raw.lines()
        .map(|l| parse_number(&l.chars().collect_vec()).1)
        .collect()
}

fn reduce(mut num: Number) -> Number {
    fn explode_down_left(number: Number, i: i32) -> Number {
        match number {
            Literal(x) => Literal(x + i),
            Pair(a, b) => Pair(Box::new(explode_down_left(*a, i)), b),
        }
    }
    fn explode_down_right(number: Number, i: i32) -> Number {
        match number {
            Literal(x) => Literal(x + i),
            Pair(a, b) => Pair(a, Box::new(explode_down_right(*b, i))),
        }
    }
    fn explode(num: Number, level: i32) -> (Number, Option<(i32, i32)>) {
        match num {
            x @ Literal(_) => (x, None),
            Pair(a, b) => {
                if level == 4 {
                    match (*a, *b) {
                        (Literal(a), Literal(b)) => (Literal(0), Some((a, b))),
                        _ => panic!(),
                    }
                } else {
                    let (a, explode_a) = explode(*a, level + 1);
                    if let Some((explode_left, explode_right)) = explode_a {
                        return (
                            Pair(Box::new(a), Box::new(explode_down_left(*b, explode_right))),
                            Some((explode_left, 0)),
                        );
                    }
                    let (b, explode_b) = explode(*b, level + 1);
                    if let Some((explode_left, explode_right)) = explode_b {
                        return (
                            Pair(Box::new(explode_down_right(a, explode_left)), Box::new(b)),
                            Some((0, explode_right)),
                        );
                    }
                    (Pair(Box::new(a), Box::new(b)), None)
                }
            }
        }
    }

    fn split(num: Number) -> (Number, bool) {
        match num {
            Literal(x) => {
                if x >= 10 {
                    let round_up = x % 2;
                    (Pair(Box::new(Literal(x / 2)), Box::new(Literal(x / 2 + round_up))), true)
                } else {
                    (Literal(x), false)
                }
            }
            Pair(a, b) => {
                let (a, q) = split(*a);
                if q {
                    (Pair(Box::new(a), b), q)
                } else {
                    let (b, q) = split(*b);
                    (Pair(Box::new(a), Box::new(b)), q)
                }
            }
        }
    }
    loop {
        let mut reduced = explode(num.clone(), 0).0;
        if num == reduced {
            let (n, q) = split(reduced);
            reduced = n;
            if !q {
                num = reduced;
                break;
            }
        }
        num = reduced;
    }
    num
}

fn add(left: Number, right: Number) -> Number {
    let added = Pair(Box::new(left), Box::new(right));
    reduce(added)
}

fn magnitude(num: Number) -> i32 {
    match num {
        Literal(x) => x,
        Pair(a, b) => 3 * magnitude(*a) + 2 * magnitude(*b),
    }
}

fn main() {
    let numbers = input();
    let added = numbers.iter().cloned().reduce(add).unwrap();
    let part1 = magnitude(added);
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    for i in numbers.iter().cloned() {
        for j in numbers.iter().cloned().filter(|x| *x != i) {
            part2 = std::cmp::max(part2, magnitude(add(i.clone(), j)))
        }
    }
    println!("Part 2: {}", part2)
}
