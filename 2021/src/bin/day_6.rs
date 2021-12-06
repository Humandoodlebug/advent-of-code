#![feature(slice_group_by)]

use std::collections::HashMap;

extern crate util;

fn main() {
    let mut  inp: Vec<i32> = util::get_day_input(6)
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    inp.sort_unstable();

    let mut all_fish = inp.clone();
    for _day in 0..80 {
        let mut new_fish = Vec::new();
        for fish in all_fish.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_fish.push(8);
            } else {
                *fish -= 1;
            }
        }
        all_fish.append(&mut new_fish);
    }

    println!("Part 1: {}", all_fish.len());
    let groups = inp.group_by(|x, y| x == y);
    for g in groups {
        dbg!(g[0]);
        dbg!(inp.clone().into_iter().filter(|&x| x == g[0]).count());
        dbg!(g.len());
    }
    let mut fish_map: HashMap<i32, i128> = inp.group_by(|x, y| x == y).map(|x| (x[0], x.len() as i128)).collect();

    for _day in 0..256 {
        dbg!(fish_map.values().sum::<i128>());
        let mut new_fish_map = HashMap::new();
        for i in 1..=8 {
            let &fish = fish_map.get(&i).unwrap_or(&0);
            new_fish_map.insert(i-1, fish);
        }
        let &births = fish_map.get(&0).unwrap_or(&0);
        *new_fish_map.get_mut(&6).unwrap() += births;
        new_fish_map.insert(8, births);
        fish_map = new_fish_map;
    }
    println!("Part 2: {}", fish_map.values().sum::<i128>())
}
