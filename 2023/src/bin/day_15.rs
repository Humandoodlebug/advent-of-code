use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<String> {
    util::get_day_input(15)
        .trim()
        .split(',')
        .map(str::to_string)
        .collect()
}

fn to_hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |s, c| ((s + c) * 17) % 256)
}

#[derive(Clone, Copy, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

#[derive(Clone, Copy, Debug)]
enum Instruction<'a> {
    Add { label: &'a str, focal_length: u8 },
    Remove { label: &'a str },
}

fn main() {
    let steps = input();
    assert!(steps.iter().all(|s| s.is_ascii()));
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = steps.iter().map(|s| to_hash(s)).sum::<usize>();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let instructions = steps.iter().map(|s| {
            if let Some((label, focal_length)) = s.split_once('=') {
                Instruction::Add {
                    label,
                    focal_length: focal_length.parse().unwrap(),
                }
            } else if s.ends_with('-') {
                Instruction::Remove {
                    label: &s[..s.len() - 1],
                }
            } else {
                panic!();
            }
        });
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        for instruction in instructions {
            match instruction {
                Instruction::Add {
                    label,
                    focal_length,
                } => {
                    let b = &mut boxes[to_hash(label)];
                    if let Some((index, _)) = b.iter().find_position(|l| l.label == label) {
                        b[index].focal_length = focal_length;
                    } else {
                        b.push(Lens {
                            label,
                            focal_length,
                        })
                    }
                }
                Instruction::Remove { label } => {
                    let b = &mut boxes[to_hash(label)];
                    if let Some((index, _)) = b.iter().find_position(|l| l.label == label) {
                        b.remove(index);
                    }
                }
            }
        }
        let part_2: u64 = boxes
            .iter()
            .enumerate()
            .map(|(box_index, b)| {
                b.iter()
                    .zip(1..)
                    .map(|(l, i)| (box_index as u64 + 1) * i * l.focal_length as u64)
                    .sum::<u64>()
            })
            .sum();
        println!("Part 2: {part_2}");
    }
}
