use std::collections::HashMap;

use regex::Regex;
use util::PerfTimer;

#[derive(Clone, Debug)]
enum Job {
    Number(i64),
    Operation { a: String, b: String, op: Op },
}

#[derive(Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn input() -> HashMap<String, Job> {
    let raw = util::get_day_input(21);
    let regex = Regex::new(
        r"^(?P<key>[a-z]+): (?:(?P<number>\d+)|(?:(?P<a>[a-z]+) (?P<op>.) (?P<b>[a-z]+)))$",
    )
    .unwrap();
    raw.lines()
        .map(|line| {
            let groups = regex.captures(line).unwrap();
            let key = groups["key"].to_string();
            let job = if let Some(num) = groups.name("number") {
                Job::Number(num.as_str().parse().unwrap())
            } else {
                let a = groups["a"].to_string();
                let b = groups["b"].to_string();
                let op = match &groups["op"] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => panic!(),
                };

                Job::Operation { a, b, op }
            };
            (key, job)
        })
        .collect()
}

fn calculate(jobs: &HashMap<String, Job>, key: &str) -> Option<i64> {
    match jobs.get(key)? {
        &Job::Number(x) => Some(x),
        Job::Operation { a, b, op } => {
            let a = calculate(jobs, a)?;
            let b = calculate(jobs, b)?;
            Some(match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mul => a * b,
                Op::Div => a / b,
            })
        }
    }
}

fn drill(jobs: &HashMap<String, Job>, target_key: &str) -> i64 {
    fn inner(jobs: &HashMap<String, Job>, target_key: &str, value: i64, key: &str) -> i64 {
        if key == target_key {
            return value;
        }
        match &jobs[key] {
            Job::Number(_) => unreachable!(),
            Job::Operation { a, b, op } => {
                let a_result = calculate(jobs, a);
                let b_result = calculate(jobs, b);
                match (a_result, b_result) {
                    (Some(a_value), None) => {
                        let b_value = match op {
                            Op::Add => value - a_value,
                            Op::Sub => a_value - value,
                            Op::Mul => value / a_value,
                            Op::Div => a_value / value,
                        };
                        inner(jobs, target_key, b_value, b)
                    }
                    (None, Some(b_value)) => {
                        let a_value = match op {
                            Op::Add => value - b_value,
                            Op::Sub => b_value + value,
                            Op::Mul => value / b_value,
                            Op::Div => b_value * value,
                        };

                        inner(jobs, target_key, a_value, a)
                    }
                    _ => panic!(),
                }
            }
        }
    }
    let mut jobs = jobs.clone();
    jobs.remove(target_key);
    let Some(Job::Operation { op, .. }) = &mut jobs.get_mut("root") else {
        panic!()
    };
    *op = Op::Sub;
    inner(&jobs, target_key, 0, "root")
}

fn main() {
    let jobs = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = calculate(&jobs, "root").unwrap();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = drill(&jobs, "humn");
        println!("Part 2: {part_2}");
    }
}
