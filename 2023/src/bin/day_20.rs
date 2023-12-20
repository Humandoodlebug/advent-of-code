use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ModuleInfo {
    name: String,
    typ: ModuleType,
    outputs: Vec<String>,
}

fn input() -> Vec<ModuleInfo> {
    util::get_day_input(20)
        .lines()
        .map(|line| {
            let (name_and_type, rest) = line.split_once(" -> ").unwrap();
            if name_and_type == "broadcaster" {
                ModuleInfo {
                    name: String::from("broadcaster"),
                    typ: ModuleType::Broadcaster,
                    outputs: rest.split(", ").map(String::from).collect(),
                }
            } else {
                let name = &name_and_type[1..];
                let typ = match name_and_type.chars().next().unwrap() {
                    '%' => ModuleType::FlipFlop,
                    '&' => ModuleType::Conjunction,
                    _ => panic!(),
                };
                ModuleInfo {
                    name: String::from(name),
                    typ,
                    outputs: rest.split(", ").map(String::from).collect(),
                }
            }
        })
        .collect()
}

#[derive(Debug)]
enum ModuleState<'a> {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

impl ModuleState<'_> {
    fn pulse(&mut self, source: &str, high: bool) -> Option<bool> {
        match self {
            ModuleState::Broadcaster => Some(high),
            ModuleState::FlipFlop(b) => {
                if high {
                    None
                } else {
                    *b = !*b;
                    Some(*b)
                }
            }
            ModuleState::Conjunction(m) => {
                *m.get_mut(source).unwrap() = high;
                if m.values().all(|b| *b) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
        }
    }
}

#[derive(Debug)]
struct Module<'a> {
    state: ModuleState<'a>,
    outputs: &'a [String],
}

impl<'a> Module<'a> {
    fn pulse(&mut self, source: &str, high: bool) -> Vec<(&'a str, bool)> {
        if let Some(b) = self.state.pulse(source, high) {
            self.outputs.iter().map(|o| (o.as_str(), b)).collect()
        } else {
            Vec::new()
        }
    }
}

fn build_modules(module_info: &[ModuleInfo]) -> HashMap<&str, Module> {
    let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    for m in module_info {
        for output in &m.outputs {
            module_inputs
                .entry(output.as_str())
                .or_default()
                .push(m.name.as_str());
        }
    }

    module_info
        .iter()
        .map(|m| {
            let state = match m.typ {
                ModuleType::Broadcaster => ModuleState::Broadcaster,
                ModuleType::FlipFlop => ModuleState::FlipFlop(false),
                ModuleType::Conjunction => ModuleState::Conjunction(HashMap::from_iter(
                    module_inputs[m.name.as_str()].iter().map(|s| (*s, false)),
                )),
            };
            let outputs = &m.outputs;
            (m.name.as_str(), Module { state, outputs })
        })
        .collect()
}

struct Pulse<'a> {
    origin: &'a str,
    destination: &'a str,
    high: bool,
}

fn count_pulses(module_info: &[ModuleInfo]) -> (u64, u64) {
    let mut modules = build_modules(module_info);
    let mut to_process = VecDeque::new();

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    for _ in 0..1000 {
        to_process.push_back(Pulse {
            origin: "button",
            destination: "broadcaster",
            high: false,
        });
        while let Some(Pulse {
            origin,
            destination,
            high,
        }) = to_process.pop_front()
        {
            if high {
                high_pulse_count += 1;
            } else {
                low_pulse_count += 1;
            }

            let Some(module) = modules.get_mut(destination) else {
                continue;
            };
            for (output, b) in module.pulse(origin, high) {
                to_process.push_back(Pulse {
                    origin: destination,
                    destination: output,
                    high: b,
                })
            }
        }
    }

    (high_pulse_count, low_pulse_count)
}

fn count_presses_to_high(module_info: &[ModuleInfo], search_for: &str) -> u64 {
    let mut modules = build_modules(module_info);
    let mut to_process = VecDeque::new();

    for presses in 1.. {
        to_process.push_back(Pulse {
            origin: "button",
            destination: "broadcaster",
            high: false,
        });
        while let Some(Pulse {
            origin,
            destination,
            high,
        }) = to_process.pop_front()
        {
            if origin == search_for && high {
                return presses;
            }
            let Some(module) = modules.get_mut(destination) else {
                continue;
            };
            for (output, b) in module.pulse(origin, high) {
                to_process.push_back(Pulse {
                    origin: destination,
                    destination: output,
                    high: b,
                })
            }
        }
    }
    unreachable!();
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
    let module_info = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let (high_pulse_count, low_pulse_count) = count_pulses(&module_info);
        let part_1 = high_pulse_count * low_pulse_count;
        dbg!(low_pulse_count, high_pulse_count);
        println!("Part 1: {}", part_1);
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let must_go_low = module_info
            .iter()
            .filter(|m| m.outputs.iter().any(|o| o == "rx"))
            .collect_vec();
        assert!(must_go_low.len() == 1);
        assert!(must_go_low[0].typ == ModuleType::Conjunction);
        let must_go_high = module_info
            .iter()
            .filter(|m| m.outputs.contains(&must_go_low[0].name))
            .map(|m| m.name.as_str())
            .collect_vec();
        let goes_high = must_go_high
            .iter()
            .map(|s| count_presses_to_high(&module_info, s))
            .collect_vec();
        let part_2 = goes_high.into_iter().reduce(lcm).unwrap();
        println!("Part 2: {}", part_2);
    }
}
