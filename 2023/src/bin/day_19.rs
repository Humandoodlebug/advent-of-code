use std::collections::HashMap;

use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Var {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Workflow {
    rules: Vec<Rule>,
    otherwise: Destination,
}

impl Workflow {
    fn process(&self, part: Part) -> &Destination {
        for rule in &self.rules {
            if rule.is_met(part) {
                return &rule.destination;
            }
        }
        &self.otherwise
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    var: Var,
    operator: Operator,
    value: i64,
    destination: Destination,
}

impl Rule {
    fn is_met(&self, part: Part) -> bool {
        let value = match self.var {
            Var::X => part.x,
            Var::M => part.m,
            Var::A => part.a,
            Var::S => part.s,
        };
        match self.operator {
            Operator::GreaterThan => value > self.value,
            Operator::LessThan => value < self.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl Rule {}

fn input() -> (HashMap<String, Workflow>, Vec<Part>) {
    let raw = util::get_day_input(19);
    let mut lines = raw.lines();
    let workflows: HashMap<String, Workflow> = (&mut lines)
        .take_while(|s| !s.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rules = rest.strip_suffix('}').unwrap().split(',').collect_vec();
            let otherwise = match *rules.last().unwrap() {
                "A" => Destination::Accept,
                "R" => Destination::Reject,
                s => Destination::Workflow(s.to_string()),
            };
            let rules = rules[0..rules.len() - 1]
                .iter()
                .map(|s| {
                    let var = match &s[0..1] {
                        "x" => Var::X,
                        "m" => Var::M,
                        "a" => Var::A,
                        "s" => Var::S,
                        _ => panic!(),
                    };
                    let operator = match &s[1..2] {
                        "<" => Operator::LessThan,
                        ">" => Operator::GreaterThan,
                        _ => panic!(),
                    };
                    let (value, destination) = s[2..].split_once(':').unwrap();
                    let destination = match destination {
                        "A" => Destination::Accept,
                        "R" => Destination::Reject,
                        s => Destination::Workflow(s.to_string()),
                    };
                    let value: i64 = value.parse().unwrap();
                    Rule {
                        var,
                        operator,
                        value,
                        destination,
                    }
                })
                .collect();
            (name.to_string(), Workflow { rules, otherwise })
        })
        .collect();

    let parts = lines
        .map(|line| {
            let (x, m, a, s) = line
                .trim_matches(&['{', '}'])
                .split(',')
                .collect_tuple()
                .unwrap();
            assert!(x.starts_with("x="));
            assert!(m.starts_with("m="));
            assert!(a.starts_with("a="));
            assert!(s.starts_with("s="));
            let x = x[2..].parse().unwrap();
            let m = m[2..].parse().unwrap();
            let a = a[2..].parse().unwrap();
            let s = s[2..].parse().unwrap();
            Part { x, m, a, s }
        })
        .collect();

    (workflows, parts)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn split(self, operator: Operator, value: i64) -> (Option<Range>, Option<Range>) {
        match operator {
            Operator::GreaterThan => {
                if self.min > value {
                    (Some(self), None)
                } else if self.max <= value {
                    (None, Some(self))
                } else {
                    (
                        Some(Range {
                            min: value + 1,
                            max: self.max,
                        }),
                        Some(Range {
                            min: self.min,
                            max: value,
                        }),
                    )
                }
            }
            Operator::LessThan => {
                if self.max < value {
                    (Some(self), None)
                } else if self.min >= value {
                    (None, Some(self))
                } else {
                    (
                        Some(Range {
                            min: self.min,
                            max: value - 1,
                        }),
                        Some(Range {
                            min: value,
                            max: self.max,
                        }),
                    )
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn get_range(&self, var: Var) -> Range {
        match var {
            Var::X => self.x,
            Var::M => self.m,
            Var::A => self.a,
            Var::S => self.s,
        }
    }
    fn get_range_mut(&mut self, var: Var) -> &mut Range {
        match var {
            Var::X => &mut self.x,
            Var::M => &mut self.m,
            Var::A => &mut self.a,
            Var::S => &mut self.s,
        }
    }

    fn split(self, rule: &Rule) -> (Option<PartRange>, Option<PartRange>) {
        let range = self.get_range(rule.var);
        let (matched, remainder) = range.split(rule.operator, rule.value);
        let matched = matched.map(|matched| {
            let mut new = self;
            *new.get_range_mut(rule.var) = matched;
            new
        });
        let remainder = remainder.map(|remainder| {
            let mut new = self;
            *new.get_range_mut(rule.var) = remainder;
            new
        });
        (matched, remainder)
    }
}

fn pipeline(workflows: &HashMap<String, Workflow>, part: Part) -> bool {
    let mut current_workflow = workflows.get("in").unwrap();
    loop {
        match current_workflow.process(part) {
            Destination::Accept => return true,
            Destination::Reject => return false,
            Destination::Workflow(name) => {
                current_workflow = &workflows[name.as_str()];
            }
        }
    }
}

fn pipeline_splits(
    workflows: &HashMap<String, Workflow>,
    part_ranges: &[PartRange],
) -> Vec<PartRange> {
    let mut to_process = part_ranges
        .iter()
        .map(|split| (*split, &workflows["in"]))
        .collect_vec();
    let mut accepted = Vec::new();
    while let Some((part_range, workflow)) = to_process.pop() {
        let mut remainder = Some(part_range);
        for rule in &workflow.rules {
            let (matched, rem) = remainder.unwrap().split(rule);
            if let Some(matched) = matched {
                match &rule.destination {
                    Destination::Accept => {
                        accepted.push(matched);
                    }
                    Destination::Reject => {}
                    Destination::Workflow(workflow_name) => {
                        to_process.push((matched, &workflows[workflow_name.as_str()]));
                    }
                }
            }
            remainder = rem;
            if remainder.is_none() {
                break;
            }
        }
        if let Some(remainder) = remainder {
            match &workflow.otherwise {
                Destination::Accept => {
                    accepted.push(remainder);
                }
                Destination::Reject => {}
                Destination::Workflow(workflow_name) => {
                    to_process.push((remainder, &workflows[workflow_name.as_str()]));
                }
            }
        }
    }
    accepted
}

fn main() {
    let (workflows, parts) = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: i64 = parts
            .iter()
            .filter(|&&part| pipeline(&workflows, part))
            .map(|&part| part.x + part.m + part.a + part.s)
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let accepted_part_ranges = pipeline_splits(
            &workflows,
            &[PartRange {
                x: Range { min: 1, max: 4000 },
                m: Range { min: 1, max: 4000 },
                a: Range { min: 1, max: 4000 },
                s: Range { min: 1, max: 4000 },
            }],
        );
        let part_2: i64 = accepted_part_ranges
            .iter()
            .map(|part_range| {
                (part_range.x.max - part_range.x.min + 1)
                    * (part_range.m.max - part_range.m.min + 1)
                    * (part_range.a.max - part_range.a.min + 1)
                    * (part_range.s.max - part_range.s.min + 1)
            })
            .sum();
        println!("Part 2: {part_2}");
    }
}
