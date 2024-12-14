use std::collections::HashMap;

use util::PerfTimer;

fn input() -> Vec<Vec<char>> {
    util::get_day_input(12)
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[allow(clippy::type_complexity)]
fn build_regions_and_plots(
    map: &[Vec<char>],
) -> (
    HashMap<(usize, usize), Vec<(usize, usize)>>,
    HashMap<(usize, usize), (usize, usize)>,
) {
    let mut regions: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut plots_to_region: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, &plot) in row.iter().enumerate() {
            if y > 0 && map[y - 1][x] == plot {
                let region = plots_to_region[&(y - 1, x)];
                plots_to_region.insert((y, x), region);
                regions.get_mut(&region).unwrap().push((y, x));

                if x > 0 && map[y][x - 1] == plot {
                    let left_region = plots_to_region[&(y, x - 1)];
                    if left_region != region {
                        let left_region = regions.remove(&left_region).unwrap();
                        for &(y, x) in &left_region {
                            plots_to_region.insert((y, x), region);
                        }
                        regions.get_mut(&region).unwrap().extend(left_region);
                    }
                }
            } else if x > 0 && map[y][x - 1] == plot {
                let region = plots_to_region[&(y, x - 1)];
                plots_to_region.insert((y, x), region);
                regions.get_mut(&region).unwrap().push((y, x));
            } else {
                let region = (y, x);
                plots_to_region.insert((y, x), region);
                regions.insert(region, vec![(y, x)]);
            }
        }
    }

    (regions, plots_to_region)
}

fn main() {
    let map = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let (regions, _plots_to_region) = build_regions_and_plots(&map);

        let part_1: usize = regions
            .keys()
            .map(|region| {
                let plot = map[region.0][region.1];
                let area = regions[region].len();
                let mut perimeter = 0;

                for &(y, x) in &regions[region] {
                    if y == 0 || map[y - 1][x] != plot {
                        perimeter += 1;
                    }
                    if x == 0 || map[y][x - 1] != plot {
                        perimeter += 1;
                    }
                    if y == map.len() - 1 || map[y + 1][x] != plot {
                        perimeter += 1;
                    }
                    if x == map[y].len() - 1 || map[y][x + 1] != plot {
                        perimeter += 1;
                    }
                }
                area * perimeter
            })
            .sum();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");

        let (regions, plots_to_region) = build_regions_and_plots(&map);
        let mut regions_to_sides: HashMap<(usize, usize), usize> = HashMap::new();

        #[allow(clippy::needless_range_loop)]
        for y in 0..map.len() {
            let mut was_in_run_above = false;
            let mut was_in_run_below = false;
            let mut prev: Option<char> = None;
            for x in 0..map[y].len() {
                let region = plots_to_region[&(y, x)];
                let plot = map[y][x];
                if prev != Some(plot) {
                    was_in_run_above = false;
                    was_in_run_below = false;
                }

                if y == 0 || map[y - 1][x] != plot {
                    if !was_in_run_above {
                        *regions_to_sides.entry(region).or_default() += 1;
                    }
                    was_in_run_above = true;
                } else {
                    was_in_run_above = false;
                }

                if y == map.len() - 1 || map[y + 1][x] != plot {
                    if !was_in_run_below {
                        *regions_to_sides.entry(region).or_default() += 1;
                    }
                    was_in_run_below = true;
                } else {
                    was_in_run_below = false;
                }

                prev = Some(plot);
            }
        }

        for x in 0..map[0].len() {
            let mut was_in_run_left = false;
            let mut was_in_run_right = false;
            let mut prev: Option<char> = None;
            for y in 0..map.len() {
                let region = plots_to_region[&(y, x)];
                let plot = map[y][x];
                if prev != Some(plot) {
                    was_in_run_left = false;
                    was_in_run_right = false;
                }

                if x == 0 || map[y][x - 1] != plot {
                    if !was_in_run_left {
                        *regions_to_sides.entry(region).or_default() += 1;
                    }
                    was_in_run_left = true;
                } else {
                    was_in_run_left = false;
                }

                if x == map[y].len() - 1 || map[y][x + 1] != plot {
                    if !was_in_run_right {
                        *regions_to_sides.entry(region).or_default() += 1;
                    }
                    was_in_run_right = true;
                } else {
                    was_in_run_right = false;
                }

                prev = Some(plot);
            }
        }

        let part_2: usize = regions_to_sides
            .iter()
            .map(|(region, sides)| regions[region].len() * sides)
            .sum();

        println!("Part 2: {part_2}");
    }
}
