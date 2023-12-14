use indexmap::IndexSet;
use itertools::Itertools;
use util::PerfTimer;

#[allow(clippy::type_complexity)]
fn input() -> (
    (usize, usize),
    IndexSet<(usize, usize)>,
    IndexSet<(usize, usize)>,
) {
    let raw = util::get_day_input(14);
    let rows = raw.lines().collect_vec();
    let dimensions = (rows.len(), rows[0].len());
    let mut cubes = IndexSet::new();
    let mut rounded = IndexSet::new();
    for (i, row) in rows.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    cubes.insert((i, j));
                }
                'O' => {
                    rounded.insert((i, j));
                }
                _ => panic!(),
            }
        }
    }
    (dimensions, cubes, rounded)
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn tilt(
    dimensions: &(usize, usize),
    cubes: &IndexSet<(usize, usize)>,
    rounded: &mut IndexSet<(usize, usize)>,
    direction: Direction,
) {
    let mv: fn(usize, usize) -> (usize, usize) = match direction {
        Direction::North => |i, j| (i - 1, j),
        Direction::East => |i, j| (i, j + 1),
        Direction::South => |i, j| (i + 1, j),
        Direction::West => |i, j| (i, j - 1),
    };
    let bounds_check: Box<dyn Fn(usize, usize) -> bool> = match direction {
        Direction::North => Box::new(|i, _j| i > 0),
        Direction::East => Box::new(|_i, j| j < dimensions.1 - 1),
        Direction::South => Box::new(|i, _j| i < dimensions.0 - 1),
        Direction::West => Box::new(|_i, j| j > 0),
    };

    let sort_key: fn(&(usize, usize)) -> usize = match direction {
        Direction::North => |&(i, _j)| i,
        Direction::East => |&(_i, j)| usize::MAX - j,
        Direction::South => |(i, _j)| usize::MAX - i,
        Direction::West => |&(_i, j)| j,
    };

    rounded.sort_by_cached_key(sort_key);

    for x in 0..rounded.len() {
        let (mut i, mut j) = rounded[x];
        while bounds_check(i, j) && !cubes.contains(&mv(i, j)) && !rounded.contains(&mv(i, j)) {
            (i, j) = mv(i, j)
        }
        // rounded[x] = (i, j);
        if rounded.insert((i, j)) {
            rounded.swap_remove_index(x);
        }
    }
}

fn main() {
    let (dimensions, cubes, rounded) = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut north_rounded = IndexSet::new();
        for &(i, j) in &rounded {
            let mut i_x = i;
            while i_x > 0
                && !cubes.contains(&(i_x - 1, j))
                && !north_rounded.contains(&(i_x - 1, j))
            {
                i_x -= 1;
            }
            north_rounded.insert((i_x, j));
        }
        let part_1: usize = north_rounded
            .into_iter()
            .map(|(i, _j)| dimensions.0 - i)
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut rounded = rounded;
        rounded.sort();
        let mut past_states = IndexSet::new();
        let initial_state = rounded.iter().copied().collect_vec();
        past_states.insert(initial_state);
        for _ in 0..1_000_000_000 {
            tilt(&dimensions, &cubes, &mut rounded, Direction::North);
            tilt(&dimensions, &cubes, &mut rounded, Direction::West);
            tilt(&dimensions, &cubes, &mut rounded, Direction::South);
            tilt(&dimensions, &cubes, &mut rounded, Direction::East);
            rounded.sort();
            let state = rounded.iter().copied().collect_vec();
            if let (loop_start, false) = past_states.insert_full(state) {
                let loop_repeat = past_states.len();
                let loop_len = loop_repeat - loop_start;
                let loop_offset = (1000000000 - loop_start) % loop_len;
                let final_state = past_states.get_index(loop_start + loop_offset).unwrap();
                let part_2: usize = final_state.iter().map(|(i, _j)| dimensions.0 - i).sum();
                println!("Part 2: {part_2}");
                break;
            }
        }
    }
}
