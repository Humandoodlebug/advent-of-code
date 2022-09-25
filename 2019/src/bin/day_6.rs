use std::collections::HashMap;

use itertools::Itertools;
use util::{get_day_input, PerfTimer};

fn input() -> Vec<(String, String)> {
    let raw = get_day_input(6);
    let mut orbits = Vec::new();
    for line in raw.lines() {
        let (orbited, orbiter) = line.split_once(')').unwrap();
        orbits.push((String::from(orbited), String::from(orbiter)));
    }
    orbits
}

fn main() {
    let orbits = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut orbit_count = 0;
        let orbits = orbits.iter().cloned().into_group_map();
        let mut set = vec![("COM", 1)];
        let mut new_set = Vec::new();
        while !set.is_empty() {
            for (orbited, value) in set {
                let orbiters = orbits.get(orbited);
                if let Some(orbiters) = orbiters {
                    for orbiter in orbiters {
                        orbit_count += value;
                        new_set.push((orbiter.as_str(), value + 1));
                    }
                }
            }
            set = new_set;
            new_set = Vec::new();
        }
        println!("Part 1: {orbit_count}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let orbits: HashMap<String, String> = orbits.iter().cloned().map(|(a, b)| (b, a)).collect();

        let mut you_path_to_com = Vec::new();
        you_path_to_com.push(orbits["YOU"].as_str());
        while *you_path_to_com.last().unwrap() != "COM" {
            you_path_to_com.push(orbits[*you_path_to_com.last().unwrap()].as_str())
        }

        let mut san_path_to_com = Vec::new();
        san_path_to_com.push(orbits["SAN"].as_str());
        while *san_path_to_com.last().unwrap() != "COM" {
            san_path_to_com.push(orbits[*san_path_to_com.last().unwrap()].as_str())
        }

        for (i, &o) in you_path_to_com.iter().enumerate() {
            if let Some(j) = san_path_to_com.iter().position(|&x| x == o) {
                let part_2 = i + j;
                println!("Part 2: {part_2}");
                break;
            }
        }
    }
}
