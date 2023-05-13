use std::{cell::RefCell, collections::BinaryHeap};

use rayon::prelude::*;
use regex::Regex;
use smallvec::{smallvec, SmallVec};
use util::PerfTimer;

#[derive(Clone, Copy)]
struct Recipe {
    ore: i64,
    clay: i64,
    obsidian: i64,
}

impl Recipe {
    /// Whether, with the currently constructed robots, we will eventually be able to afford this recipe.
    fn approaching_recipe(&self, state: &State) -> bool {
        (self.ore == 0 || state.ore_robots > 0)
            && (self.clay == 0 || state.clay_robots > 0)
            && (self.obsidian == 0 || state.obsidian_robots > 0)
    }
}

#[derive(Clone, Copy)]
struct Blueprint {
    id: i64,
    ore_robot_recipe: Recipe,
    clay_robot_recipe: Recipe,
    obsidian_robot_recipe: Recipe,
    geode_robot_recipe: Recipe,
}

fn input() -> Vec<Blueprint> {
    let blueprint_regex = Regex::new(r#"^Blueprint (\d+): (.+)\.$"#).unwrap();
    let recipe_regex = Regex::new(
        r#"^Each (?P<robot>\w+) robot costs (?P<ore>\d+) ore(?: and (?:(?P<clay>\d+) clay|(?P<obsidian>\d+) obsidian))?$"#,
    ).unwrap();

    let raw = util::get_day_input(19);
    raw.lines()
        .map(|line| {
            let blueprint_match = blueprint_regex.captures(line).unwrap();
            let id = blueprint_match[1].parse().unwrap();

            let mut ore_robot_recipe = None;
            let mut clay_robot_recipe = None;
            let mut obsidian_robot_recipe = None;
            let mut geode_robot_recipe = None;

            for recipe in blueprint_match[2].split(". ") {
                let captures = recipe_regex.captures(recipe).unwrap();
                let robot = match &captures["robot"] {
                    "ore" => &mut ore_robot_recipe,
                    "clay" => &mut clay_robot_recipe,
                    "obsidian" => &mut obsidian_robot_recipe,
                    "geode" => &mut geode_robot_recipe,
                    other => panic!("Unknown robot type {other:?}"),
                };

                let ore = captures["ore"].parse().unwrap();
                let clay = captures
                    .name("clay")
                    .map(|m| m.as_str().parse().unwrap())
                    .unwrap_or(0);
                let obsidian = captures
                    .name("obsidian")
                    .map(|m| m.as_str().parse().unwrap())
                    .unwrap_or(0);

                *robot = Some(Recipe {
                    ore,
                    clay,
                    obsidian,
                });
            }

            Blueprint {
                id,
                ore_robot_recipe: ore_robot_recipe.unwrap(),
                clay_robot_recipe: clay_robot_recipe.unwrap(),
                obsidian_robot_recipe: obsidian_robot_recipe.unwrap(),
                geode_robot_recipe: geode_robot_recipe.unwrap(),
            }
        })
        .collect()
}

struct State<'a> {
    blueprint: &'a Blueprint,
    time_left: i64,
    ore: i64,
    clay: i64,
    obsidian: i64,
    geodes: i64,
    ore_robots: i64,
    clay_robots: i64,
    obsidian_robots: i64,
    geode_robots: i64,
    upper_bound: RefCell<Option<i64>>,
    do_not_build: [bool; 4],
}

impl<'a> Clone for State<'a> {
    fn clone(&self) -> Self {
        Self {
            upper_bound: RefCell::new(None),
            ..*self
        }
    }
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.blueprint, other.blueprint)
            && self.time_left == other.time_left
            && self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geodes == other.geodes
            && self.ore_robots == other.ore_robots
            && self.clay_robots == other.clay_robots
            && self.obsidian_robots == other.obsidian_robots
            && self.geode_robots == other.geode_robots
            && self.do_not_build == other.do_not_build
    }
}

impl<'a> Eq for State<'a> {}

impl<'a> State<'a> {
    fn make_recipe<'b>(&'b self, recipe: &'a Recipe) -> Option<State<'a>> {
        let mut new = self.clone();
        new.do_not_build = [false; 4];
        new.ore -= recipe.ore;
        new.clay -= recipe.clay;
        new.obsidian -= recipe.obsidian;

        if new.ore >= 0 && new.clay >= 0 && new.obsidian >= 0 {
            Some(new)
        } else {
            None
        }
    }

    fn next<'b>(&'b self) -> SmallVec<[State<'a>; 5]> {
        if self.blueprint.geode_robot_recipe.obsidian <= self.obsidian_robots.min(self.obsidian)
            && self.blueprint.geode_robot_recipe.clay <= self.clay_robots.min(self.clay)
            && self.blueprint.geode_robot_recipe.ore <= self.ore_robots.min(self.ore)
        {
            if self.time_left == 1 {
                return smallvec![State {
                    time_left: 0,
                    geodes: self.geodes + self.geode_robots,
                    upper_bound: RefCell::new(None),
                    ..*self
                }];
            }

            let geodes = self.geodes
                + self.geode_robots * self.time_left
                + (self.time_left * (self.time_left - 1)) / 2;

            return smallvec![State {
                time_left: 0,
                geodes,
                geode_robots: self.geode_robots + self.time_left,
                upper_bound: RefCell::new(Some(geodes)),
                ..*self
            }];
        }

        let mut new_states: SmallVec<[State<'a>; 5]> = SmallVec::new();
        let mut might_wait = false;

        let mut relevant_recipes: SmallVec<[&Recipe; 5]> = SmallVec::new();
        relevant_recipes.push(&self.blueprint.geode_robot_recipe);

        let obsidian_relevant =
            self.obsidian_robots < relevant_recipes.iter().map(|r| r.obsidian).max().unwrap();

        if obsidian_relevant {
            relevant_recipes.push(&self.blueprint.obsidian_robot_recipe);
        }

        let clay_relevant =
            self.clay_robots < relevant_recipes.iter().map(|r| r.clay).max().unwrap();

        if clay_relevant {
            relevant_recipes.push(&self.blueprint.clay_robot_recipe)
        }

        let ore_relevant = self.ore_robots < relevant_recipes.iter().map(|r| r.ore).max().unwrap();

        drop(relevant_recipes);

        if !self.do_not_build[3] {
            if let Some(mut new_state) = self.make_recipe(&self.blueprint.geode_robot_recipe) {
                new_state.geode_robots += 1;
                new_states.push(new_state);
            } else if self.blueprint.geode_robot_recipe.approaching_recipe(self) {
                might_wait = true;
            }
        }

        if !self.do_not_build[2] && obsidian_relevant {
            if let Some(mut new_state) = self.make_recipe(&self.blueprint.obsidian_robot_recipe) {
                new_state.obsidian_robots += 1;
                new_states.push(new_state);
            } else if self
                .blueprint
                .obsidian_robot_recipe
                .approaching_recipe(self)
            {
                might_wait = true;
            }
        }

        if !self.do_not_build[1] && clay_relevant {
            if let Some(mut new_state) = self.make_recipe(&self.blueprint.clay_robot_recipe) {
                new_state.clay_robots += 1;
                new_states.push(new_state);
            } else if self.blueprint.clay_robot_recipe.approaching_recipe(self) {
                might_wait = true;
            }
        }

        if !self.do_not_build[0] && ore_relevant {
            if let Some(mut new_state) = self.make_recipe(&self.blueprint.ore_robot_recipe) {
                new_state.ore_robots += 1;
                new_states.push(new_state);
            } else if self.blueprint.ore_robot_recipe.approaching_recipe(self) {
                might_wait = true;
            }
        }

        // If we can already make all of the recipes, it can't be optimal for us to make none of them.
        if might_wait || new_states.is_empty() {
            let mut new_state = self.clone();
            new_state.do_not_build = [
                self.make_recipe(&self.blueprint.ore_robot_recipe).is_some(),
                self.make_recipe(&self.blueprint.clay_robot_recipe)
                    .is_some(),
                self.make_recipe(&self.blueprint.obsidian_robot_recipe)
                    .is_some(),
                self.make_recipe(&self.blueprint.geode_robot_recipe)
                    .is_some(),
            ];
            new_states.push(new_state);
        }

        for new_state in &mut new_states {
            new_state.ore += self.ore_robots;
            new_state.clay += self.clay_robots;
            new_state.obsidian += self.obsidian_robots;
            new_state.geodes += self.geode_robots;
            new_state.time_left -= 1;
        }

        new_states
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn upper_bound(state: &State) -> i64 {
            if let Some(b) = *state.upper_bound.borrow() {
                return b;
            }

            let mut sum_state = state.clone();
            while sum_state.time_left > 0 {
                let next_states = sum_state.next();
                sum_state = State {
                    blueprint: state.blueprint,
                    time_left: next_states[0].time_left,
                    ore: next_states.iter().map(|x| x.ore).max().unwrap(),
                    clay: next_states.iter().map(|x| x.clay).max().unwrap(),
                    obsidian: next_states.iter().map(|x| x.obsidian).max().unwrap(),
                    geodes: next_states.iter().map(|x| x.geodes).max().unwrap(),
                    ore_robots: next_states.iter().map(|x| x.ore_robots).max().unwrap(),
                    clay_robots: next_states.iter().map(|x| x.clay_robots).max().unwrap(),
                    obsidian_robots: next_states.iter().map(|x| x.obsidian_robots).max().unwrap(),
                    geode_robots: next_states.iter().map(|x| x.geode_robots).max().unwrap(),
                    upper_bound: RefCell::new(None),
                    do_not_build: [false; 4],
                }
            }

            *state.upper_bound.borrow_mut() = Some(sum_state.geodes);

            sum_state.geodes

            // state.geodes
            //     + state.time_left * state.geode_robots
            //     + state.time_left * (state.time_left - 1) / 2
        }
        upper_bound(self).cmp(&upper_bound(other))
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_max_geodes<'a>(initial_state: State<'a>) -> i64 {
    let mut pq: BinaryHeap<State<'a>> = BinaryHeap::new();
    pq.push(initial_state);

    loop {
        let state = pq.pop().unwrap();
        if state.time_left == 0 {
            break state.geodes;
        }
        for state in state.next() {
            pq.push(state);
        }
    }
}

fn main() {
    let blueprints = input();
    {
        let _timer = PerfTimer::new("Part 1");

        let part_1: i64 = blueprints
            .par_iter()
            .map(|blueprint| {
                let initial_state = State {
                    blueprint,
                    time_left: 24,
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                    upper_bound: RefCell::new(None),
                    do_not_build: [false; 4],
                };
                blueprint.id * find_max_geodes(initial_state)
            })
            // .inspect(|g| println!("{g}"))
            .sum();

        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");

        let part_2: i64 = blueprints
            .par_iter()
            .take(3)
            .map(|blueprint| {
                let initial_state = State {
                    blueprint,
                    time_left: 32,
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                    upper_bound: RefCell::new(None),
                    do_not_build: [false; 4],
                };
                find_max_geodes(initial_state)
            })
            // .inspect(|g| println!("{g}"))
            .product();

        println!("Part 2: {part_2}");
    }
}
