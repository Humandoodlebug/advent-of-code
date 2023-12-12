use std::collections::HashMap;

use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

fn input() -> Vec<(Vec<Condition>, Vec<usize>)> {
    util::get_day_input(12)
        .lines()
        .map(|line| {
            let (row, groups) = line.split_once(' ').unwrap();
            let row: Vec<Condition> = row
                .chars()
                .map(|c| match c {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => panic!(),
                })
                .collect();
            let groups: Vec<usize> = groups.split(',').map(|s| s.parse()).try_collect().unwrap();
            (row, groups)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    group: usize,
    group_len: usize,
}

fn count_permutations(row: &[Condition], groups: &[usize]) -> u64 {
    let mut states: HashMap<State, u64> = HashMap::new();
    states.insert(
        State {
            group: 0,
            group_len: 0,
        },
        1,
    );
    for &spring in row.iter().chain(&[Condition::Operational]) {
        let mut new_states: HashMap<State, u64> = HashMap::new();
        for (state, count) in states {
            if (spring == Condition::Damaged || spring == Condition::Unknown)
                && state.group < groups.len()
                && state.group_len < groups[state.group]
            {
                let new_state = State {
                    group: state.group,
                    group_len: state.group_len + 1,
                };
                *new_states.entry(new_state).or_insert(0) += count;
            }
            if (spring == Condition::Operational || spring == Condition::Unknown)
                && (state.group_len == 0 || state.group_len == groups[state.group])
            {
                let new_state = State {
                    group: state.group + usize::from(state.group_len != 0),
                    group_len: 0,
                };
                *new_states.entry(new_state).or_insert(0) += count;
            }
        }
        states = new_states;
    }

    states
        .into_iter()
        .filter(|(state, _)| state.group == groups.len())
        .map(|(_, count)| count)
        .sum()
}

fn main() {
    let records = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: u64 = records
            .iter()
            .map(|(row, groups)| count_permutations(row, groups))
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let records = records
            .into_iter()
            .map(|(row, groups)| {
                (
                    row.iter()
                        .copied()
                        .chain(std::iter::once(Condition::Unknown))
                        .cycle()
                        .take((row.len() + 1) * 5 - 1)
                        .collect_vec(),
                    groups
                        .iter()
                        .copied()
                        .cycle()
                        .take(groups.len() * 5)
                        .collect_vec(),
                )
            })
            .collect_vec();

        let part_2: u64 = records
            .iter()
            .map(|(row, groups)| count_permutations(row, groups))
            .sum();
        println!("Part 2: {part_2}");
    }
}
