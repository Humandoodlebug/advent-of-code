use regex::Regex;
use std::cmp::{max, min};

extern crate util;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    on: bool,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

fn input() -> Vec<Instruction> {
    let raw = util::get_day_input(22);
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();
    raw.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let on = match &captures[1] {
                "on" => true,
                "off" => false,
                _ => panic!(),
            };
            let x_range = (captures[2].parse().unwrap(), captures[3].parse().unwrap());
            let y_range = (captures[4].parse().unwrap(), captures[5].parse().unwrap());
            let z_range = (captures[6].parse().unwrap(), captures[7].parse().unwrap());

            Instruction {
                on,
                x_range,
                y_range,
                z_range,
            }
        })
        .collect()
}

fn calc_instruction_volume(instruction: Instruction) -> i128 {
    let calc_range = |(a, b)| (b - a + 1) as i128;
    calc_range(instruction.x_range) * calc_range(instruction.y_range) * calc_range(instruction.z_range)
}

fn main() {
    let inp = input();
    // dbg!(inp);
    let rev: Vec<Instruction> = inp.iter().copied().rev().collect();
    let mut part1 = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                for inst in rev.iter() {
                    if (inst.x_range.0..=inst.x_range.1).contains(&x)
                        && (inst.y_range.0..=inst.y_range.1).contains(&y)
                        && (inst.z_range.0..=inst.z_range.1).contains(&z)
                    {
                        if inst.on {
                            part1 += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", part1);

    let mut layers: Vec<Instruction> = vec![];

    for &inst in &inp {
        let mut new_layers = vec![];
        for &layer in &layers {
            let x_min = max(inst.x_range.0, layer.x_range.0);
            let x_max = min(inst.x_range.1, layer.x_range.1);

            let y_min = max(inst.y_range.0, layer.y_range.0);
            let y_max = min(inst.y_range.1, layer.y_range.1);

            let z_min = max(inst.z_range.0, layer.z_range.0);
            let z_max = min(inst.z_range.1, layer.z_range.1);

            if x_min <= x_max && y_min <= y_max && z_min <= z_max {
                let overlay = Instruction {
                    on: !layer.on,
                    x_range: (x_min, x_max),
                    y_range: (y_min, y_max),
                    z_range: (z_min, z_max),
                };
                new_layers.push(overlay);
            }
        }
        layers.append(&mut new_layers);

        if inst.on {
            layers.push(inst);
        }
    }

    let part2: i128 = layers.into_iter().map(|l| if l.on {
        calc_instruction_volume(l)
    } else {
        -calc_instruction_volume(l)
    }).sum();
    println!("Part 2: {}", part2);
}
