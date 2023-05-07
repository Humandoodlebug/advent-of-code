use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use util::PerfTimer;

#[derive(Clone, Debug)]
struct InputValve {
    rate: u64,
    leads_to: HashMap<String, u64>,
}

fn input() -> HashMap<String, InputValve> {
    let re =
        Regex::new(
            r#"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<leads_to>(?:[A-Z]{2}, )*[A-Z]{2})$"#
        ).unwrap();

    util::get_day_input(16)
        .trim()
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let name = String::from(&captures["name"]);
            let rate = captures["rate"].parse().unwrap();
            let leads_to = captures["leads_to"]
                .split(", ")
                .map(|v| (String::from(v), 1))
                .collect();
            (name, InputValve { rate, leads_to })
        })
        .collect()
}

#[allow(dead_code)]
fn encode_path(valve_a: usize, valve_b: usize) -> usize {
    valve_a * 16 + valve_b
}

fn decode_path(path: usize) -> (usize, usize) {
    (path / 16, path % 16)
}

struct GameMap {
    rate_at: [u64; 16],
    paths: [Option<u64>; 256],
}

impl GameMap {
    fn get_paths_from(&self, valve: usize) -> &[Option<u64>] {
        let encoded_valve = valve * 16;
        &self.paths[encoded_valve..encoded_valve + 16]
    }
}

fn make_game_map(input_valves: HashMap<&str, InputValve>) -> GameMap {
    assert_eq!(input_valves.len(), 16);
    let names: Vec<&str> = input_valves.keys().copied().sorted().collect();
    let mut valves = [0_u64; 16];
    for (i, valve) in valves.iter_mut().enumerate() {
        *valve = input_valves[names[i]].rate;
    }

    let mut paths = [None; 256];
    for (i, path) in paths.iter_mut().enumerate() {
        let (valve_a, valve_b) = decode_path(i);
        *path = input_valves[names[valve_a]]
            .leads_to
            .get(names[valve_b])
            .copied();
    }

    GameMap {
        rate_at: valves,
        paths,
    }
}

fn reduce_valves(mut valves: HashMap<&str, InputValve>) -> GameMap {
    // First, remove 0-valves
    let valves_to_remove: Vec<&str> = valves
        .iter()
        .filter_map(|(name, valve)| {
            if valve.rate == 0 && *name != "AA" {
                Some(name)
            } else {
                None
            }
        })
        .cloned()
        .collect();

    for name in valves_to_remove {
        let valve = valves.remove(&name).unwrap();
        for (n, distance) in &valve.leads_to {
            let v = valves.get_mut(n.as_str()).unwrap();
            v.leads_to.remove(name);
            for (nx, &dx) in &valve.leads_to {
                if nx != n {
                    let nd = distance + dx;
                    v.leads_to
                        .entry(nx.clone())
                        .and_modify(|x| *x = nd.min(*x))
                        .or_insert(nd);
                }
            }
        }
    }

    // Second, encode valves as integers and produce a GameMap
    let mut map = make_game_map(valves);

    // Third, connect all remaining valves through their shortest path
    loop {
        let mut changed = false;

        for i in 0..16 {
            for j in 0..16 {
                let i_j_path = encode_path(i, j);
                if let Some(i_j_distance) = map.paths[i_j_path] {
                    for k in 0..16 {
                        let i_k_path = encode_path(i, k);
                        if let Some(i_k_distance) = map.paths[i_k_path] {
                            let j_k_path = encode_path(j, k);
                            let new_j_k_distance = i_j_distance + i_k_distance;
                            if let Some(j_k_distance) = map.paths[j_k_path] {
                                if new_j_k_distance < j_k_distance {
                                    map.paths[j_k_path] = Some(new_j_k_distance);
                                    changed = true;
                                }
                            } else {
                                map.paths[j_k_path] = Some(i_j_distance + i_k_distance);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    map
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GameState {
    position: usize,
    time_left: u64,
    valve_is_open: [bool; 16],
    total_released: u64,
}

#[derive(Eq, PartialEq)]
struct PQueueItem(u64, GameState);

impl PartialOrd for PQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PQueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

fn upper_bound(sorted_valves: &[(usize, u64)], state: &GameState) -> u64 {
    let mut time_left = if state.valve_is_open[state.position] {
        state.time_left
    } else {
        state.time_left + 1
    };

    if time_left < 2 {
        return state.total_released;
    }

    let mut bound = state.total_released;
    for (valve, rate) in sorted_valves.iter().copied() {
        if !state.valve_is_open[valve] {
            time_left -= 2;
            bound += rate * time_left;
            if time_left < 2 {
                break;
            }
        }
    }

    bound
}

fn find_max_path(map: &GameMap, mut initial_state: GameState) -> u64 {
    assert_eq!(map.rate_at[initial_state.position], 0);
    initial_state.valve_is_open[initial_state.position] = true;

    let sorted_valves: Vec<(usize, u64)> = map
        .rate_at
        .iter()
        .copied()
        .enumerate()
        .sorted_by_key(|&(_k, v)| v)
        .rev()
        .collect();
    let mut pq: BinaryHeap<PQueueItem> = BinaryHeap::new();
    pq.push(PQueueItem(
        upper_bound(&sorted_valves, &initial_state),
        initial_state,
    ));

    loop {
        let PQueueItem(_, state) = pq.pop().unwrap();

        if state.time_left == 0 {
            return state.total_released;
        }

        let mut stranded = true;

        for (new_pos, time_taken) in map
            .get_paths_from(state.position)
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(pos, time)| time.map(|time| (pos, time)))
            .filter(|&(pos, time)| !state.valve_is_open[pos] && time < state.time_left)
        {
            let mut new_state = state.clone();
            new_state.time_left -= time_taken + 1;
            new_state.position = new_pos;
            new_state.valve_is_open[new_pos] = true;
            new_state.total_released += new_state.time_left * map.rate_at[new_pos];
            pq.push(PQueueItem(
                upper_bound(&sorted_valves, &new_state),
                new_state,
            ));
            stranded = false;
        }

        if stranded {
            let mut new_state = state.clone();
            new_state.time_left = 0;
            pq.push(PQueueItem(
                upper_bound(&sorted_valves, &new_state),
                new_state,
            ));
        }
    }
}

fn main() {
    let valves = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let valves: HashMap<&str, InputValve> = valves
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        let map = reduce_valves(valves);

        let initial_state = GameState {
            position: 0,
            time_left: 30,
            valve_is_open: [false; 16],
            total_released: 0,
        };

        let part_1 = find_max_path(&map, initial_state);
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let valves: HashMap<&str, InputValve> = valves
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        let map = reduce_valves(valves);

        // Split the search space in two, considering each possible split.
        // We find the best paths through each half of the search space and add together the total amount of gas
        // released by the paths. We then find best
        let part_2 = (0..16)
            .powerset()
            .par_bridge()
            .map(|valves_a| {
                let mut valve_is_open_a = [true; 16];
                let mut valve_is_open_b = [false; 16];
                for v in valves_a {
                    valve_is_open_a[v] = false;
                    valve_is_open_b[v] = true;
                }

                // We don't care whether the first valve (AA) is open or not, so we only look at sets where it isn't,
                // which cuts the search space in half.
                // Since we also don't care which player (us or the elephant) is using a given path (the score will be
                // the same if you swap them), we can also ignore all paths where the elephant would consider an
                // arbitrary valve (here we've picked the valve represented by `1`) so that we always consider those
                // paths instead of the elephant, cutting the search space in half again.
                if valve_is_open_a[0] || valve_is_open_a[1] {
                    return 0;
                }

                let initial_state_a = GameState {
                    position: 0,
                    time_left: 26,
                    valve_is_open: valve_is_open_a,
                    total_released: 0,
                };
                let initial_state_b = GameState {
                    position: 0,
                    time_left: 26,
                    valve_is_open: valve_is_open_b,
                    total_released: 0,
                };

                let max_a = find_max_path(&map, initial_state_a);
                let max_b = find_max_path(&map, initial_state_b);

                max_a + max_b
            })
            .max()
            .unwrap();

        println!("Part 2: {part_2}");
    }
}
