use std::collections::{HashMap, HashSet};

extern crate util;

fn input() -> HashMap<String, HashSet<String>> {
    util::get_day_input(12)
        .lines()
        .flat_map(|l| {
            let (left, right) = l.split_once('-').unwrap();
            [(left, right), (right, left)]
        })
        .fold(HashMap::new(), |mut m, (l, r)| {
            if let Some(s) = m.get_mut(l) {
                s.insert(r.to_owned());
            } else {
                m.insert(l.to_owned(), HashSet::from([r.to_owned()]));
            }
            m
        })
}

fn main() {
    let graph = input();
    let mut paths = vec![(HashSet::<String>::new(), "start")];
    let mut path_count = 0;
    while let Some((mut visited, location)) = paths.pop() {
        visited.insert(location.to_owned());
        for cave in graph.get(location).unwrap() {
            if cave.to_lowercase() == *cave && visited.contains(cave) {
                continue;
            }
            if cave == "end" {
                path_count += 1;
                continue;
            }
            paths.push((visited.clone(), cave))
        }
    }
    println!("Part 1: {}", path_count);

    let mut paths = vec![(HashSet::<String>::new(), false, "start")];
    let mut path_count = 0;
    while let Some((mut visited, twice, location)) = paths.pop() {
        visited.insert(location.to_owned());
        for cave in graph.get(location).unwrap() {
            let mut twice = twice;
            if cave.to_lowercase() == *cave && visited.contains(cave) {
                if twice {
                    continue;
                } else {
                    twice = true;
                }
            }
            if cave == "start" {
                continue;
            }
            if cave == "end" {
                path_count += 1;
                continue;
            }
            paths.push((visited.clone(), twice, cave))
        }
    }
    println!("Part 2: {}", path_count);
}
