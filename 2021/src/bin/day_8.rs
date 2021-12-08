#![feature(hash_drain_filter)]

use std::collections::{HashMap, HashSet};

extern crate util;

struct Display {
    digits: [String; 10],
    display: [String; 4],
}

fn input() -> Vec<Display> {
    let raw = util::get_day_input(8);
    raw.lines()
        .into_iter()
        .map(|s| {
            if let Some((l, r)) = s.split_once(" | ") {
                fn read_to_arr<const N: usize>(s: &str) -> [String; N] {
                    s.split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .try_into()
                        .unwrap()
                }
                Display {
                    digits: read_to_arr(l),
                    display: read_to_arr(r),
                }
            } else {
                panic!("AHHHHHH");
            }
        })
        .collect()
}

fn main() {
    let inp = input();
    let p1_lens = [2, 4, 3, 7];
    let part1: usize = inp
        .iter()
        .map(|d| {
            d.display
                .iter()
                .filter(|s| p1_lens.contains(&s.len()))
                .count()
        })
        .sum();
    println!("Part 1: {}", part1);

    let digit_strings = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    let mut sum = 0;

    fn chars_in_digits_of_len(digits: &[String], len: usize) -> HashSet<char> {
        digits
            .iter()
            .filter(|x| x.len() == len)
            .fold(String::new(), |s, x| s + x)
            .chars()
            .collect()
    }

    for Display { digits, display } in inp {
        let mut possible_mappings: HashMap<char, HashSet<char>> =
            ('a'..='g').map(|k| (k, ('a'..='g').collect())).collect();

        for (keys, values) in (2..=7).map(|i| {
            (
                chars_in_digits_of_len(&digits, i),
                chars_in_digits_of_len(
                    &digit_strings
                        .iter()
                        .map(|&s| String::from(s))
                        .collect::<Vec<_>>(),
                    i,
                ),
            )
        }) {
            for k in 'a'..='g' {
                let v = possible_mappings.get_mut(&k).unwrap();
                if keys.contains(&k) {
                    v.drain_filter(|x| !values.contains(x));
                } else {
                    v.drain_filter(|x| values.contains(x));
                }
            }
        }

        let mut display_sum = 0;
        for d in display {
            let mut possible_displays = HashSet::new();
            possible_displays.insert(String::new());
            for s in d.chars() {
                let vs = possible_mappings.get(&s).unwrap();
                possible_displays = possible_displays
                    .into_iter()
                    .flat_map(|s| {
                        vs.iter()
                            .map(|&v| {
                                let mut sx = s.clone();
                                sx.push(v);
                                sx
                            })
                            .collect::<Vec<String>>()
                    })
                    .collect();
            }
            possible_displays
                .drain_filter(|s| HashSet::<char>::from_iter(s.chars()).len() != s.len());

            possible_displays = possible_displays
                .into_iter()
                .map(|s| {
                    let mut sx = s.chars().collect::<Vec<char>>();
                    sx.sort_unstable();
                    String::from_iter(sx)
                })
                .collect();

            possible_displays.drain_filter(|s| !digit_strings.contains(&s.as_str()));
            assert!(possible_displays.len() == 1);
            let final_display = possible_displays.into_iter().next().unwrap();
            let final_digit = digit_strings
                .iter()
                .position(|&x| x == final_display.as_str())
                .unwrap();
            display_sum = display_sum * 10 + final_digit;
        }

        sum += display_sum;
    }

    println!("Part 2: {}", sum);
}
