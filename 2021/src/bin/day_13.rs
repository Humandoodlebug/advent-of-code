use std::collections::HashSet;

use itertools::Itertools;

extern crate util;

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn input() -> (Vec<(i32, i32)>, Vec<Fold>) {
    let raw = util::get_day_input(13);
    let points = raw.lines().take_while(|l| !l.is_empty())
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let folds = raw.lines().skip_while(|l| !l.is_empty()).skip(1).map(|l| {
        let axis = &l[11..12];
        let inc = l[13..].parse().unwrap();
        match axis {
            "x" => Fold::X(inc),
            "y" => Fold::Y(inc),
            other => panic!("Unknown axis {:?}", other),
        }
    }).collect();
    (points, folds)
}

fn map_over_fold(fold: Fold, (x, y): (i32, i32)) -> (i32, i32) {
    match fold {
        Fold::X(i) => if x > i {
            (2 * i - x, y)
        } else {
            (x, y)
        }
        Fold::Y(i) => if y > i {
            (x, 2 * i - y)
        } else {
            (x, y)
        }
    }
}

fn main() {
    let (points, folds) = input();

    let part1 = points.iter().map(|&p| map_over_fold(folds[0], p)).unique().count();
    println!("Part 1: {}", part1);

    let part2: HashSet<(i32, i32)> = folds.iter().fold(Box::new(points.into_iter()) as Box<dyn Iterator<Item = (i32, i32)>>, |points: Box<dyn Iterator<Item = (i32, i32)>>, f| Box::new(points.map(|p| map_over_fold(*f, p)))).collect();
    let &(max_x, _) = part2.iter().max_by_key(|&(x, _y)| x).unwrap();
    let &(_, max_y) = part2.iter().max_by_key(|&(_x, y)| y).unwrap();

    println!("Part 2:");
    for j in 0..=max_y {
        let mut line = String::new();
        for i in 0..=max_x {
            if part2.contains(&(i, j)) {
                line.push('#');
            } else {
                line.push(' ')
            }
        }
        println!("{}", line);
    }
}
