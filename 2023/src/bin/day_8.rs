use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

enum Instruction {
    Left,
    Right,
}

fn input() -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let raw = util::get_day_input(8);
    let mut lines = raw.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!(),
        })
        .collect();
    assert_eq!(lines.next().unwrap(), "");
    let node_regex =
        Regex::new(r"(?P<name>[[:alnum:]]+) = \((?P<left>[[:alnum:]]+), (?P<right>[[:alnum:]]+)\)")
            .unwrap();
    let nodes = lines
        .map(|line| {
            let captures = node_regex.captures(line).unwrap();
            let name = String::from(&captures["name"]);
            let left = String::from(&captures["left"]);
            let right = String::from(&captures["right"]);
            (name, (left, right))
        })
        .collect();

    (instructions, nodes)
}

fn hcf(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        hcf(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / hcf(a, b)
}

fn main() {
    let (instructions, nodes) = input();
    {
        let _timer = util::PerfTimer::new("Part 1");
        let mut current = "AAA";
        let mut steps: u64 = 0;
        for instruction in instructions.iter().cycle() {
            if current == "ZZZ" {
                break;
            }
            let (left, right) = nodes.get(current).unwrap();
            current = match instruction {
                Instruction::Left => left,
                Instruction::Right => right,
            };
            steps += 1;
        }
        println!("Part 1: {steps}");
    }

    {
        let _timer = util::PerfTimer::new("Part 2");
        let mut current = nodes.keys().filter(|n| n.ends_with('A')).collect_vec();
        let mut steps: Vec<Option<u64>> = vec![None; current.len()];
        let mut target_nodes: Vec<Option<&str>> = vec![None; current.len()];
        for (step, instruction) in (0_u64..).zip(instructions.iter().cycle()) {
            for ((node, completion), end_node) in
                current.iter().zip(&mut steps).zip(&mut target_nodes)
            {
                if node.ends_with('Z') {
                    if completion.is_none() {
                        *completion = Some(step);
                        // lock target node to first target node we encounter
                        *end_node = Some(node);
                    } else if let Some(verify) = end_node {
                        let completion = completion.unwrap();
                        // Assert that we have not encountered a different target node.
                        // The algorithm needs to assume this to work.
                        assert!(verify == node);
                        // Assert that we have returned to the target node in exactly the same number
                        // of steps it took us to get to the target node in the first place.
                        // The algorithm only works if this is true!
                        assert!(step % completion == 0);
                    }
                }
            }

            if steps.iter().all(Option::is_some) {
                break;
            }

            for node in current.iter_mut() {
                let (left, right) = nodes.get(*node).unwrap();
                *node = match instruction {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }
        }
        let completions = steps.into_iter().collect::<Option<Vec<_>>>().unwrap();
        let part_2 = completions.into_iter().reduce(lcm).unwrap();
        println!("Part 2: {part_2}");
    }
}
