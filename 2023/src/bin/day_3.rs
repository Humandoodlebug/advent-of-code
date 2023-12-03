use std::collections::HashMap;

use util::PerfTimer;

fn symbol_is_adjacent(symbols: &HashMap<(usize, usize), char>, row: usize, col: usize) -> bool {
    let row_min = row.max(1) - 1;
    let row_max = row + 1;
    let col_min = col.max(1) - 1;
    let col_max = col + 1;
    for i in row_min..=row_max {
        for j in col_min..=col_max {
            if symbols.contains_key(&(i, j)) {
                return true;
            }
        }
    }
    false
}

fn find_adjacent_star(
    symbols: &HashMap<(usize, usize), char>,
    row: usize,
    col: usize,
) -> Option<(usize, usize)> {
    let row_min = row.max(1) - 1;
    let row_max = row + 1;
    let col_min = col.max(1) - 1;
    let col_max = col + 1;
    for i in row_min..=row_max {
        for j in col_min..=col_max {
            if symbols.get(&(i, j)) == Some(&'*') {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let input = util::get_day_input(3);

    {
        let _timer = PerfTimer::new("Part 1");
        let mut symbols = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' && !c.is_ascii_digit() {
                    symbols.insert((row, col), c);
                }
            }
        }

        let mut sum = 0;
        let mut num = 0;
        let mut adjacent = false;

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    num = num * 10 + c.to_digit(10).unwrap();
                    if symbol_is_adjacent(&symbols, row, col) {
                        adjacent = true;
                    }
                } else {
                    if adjacent {
                        sum += num;
                    }
                    num = 0;
                    adjacent = false;
                }
            }
            if adjacent {
                sum += num;
            }
            num = 0;
            adjacent = false;
        }
        println!("Part 1: {sum}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut symbols = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' && !c.is_ascii_digit() {
                    symbols.insert((row, col), c);
                }
            }
        }

        let mut sum = 0;
        let mut num = 0;
        let mut adjacents: HashMap<(usize, usize), u32> = HashMap::new();
        let mut adjacent: Option<(usize, usize)> = None;

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    num = num * 10 + c.to_digit(10).unwrap();
                    if let Some(pos) = find_adjacent_star(&symbols, row, col) {
                        adjacent = Some(pos);
                    }
                } else {
                    if let Some(pos) = adjacent {
                        if let Some(other_num) = adjacents.remove(&pos) {
                            sum += num * other_num;
                        } else {
                            adjacents.insert(pos, num);
                        }
                    }
                    num = 0;
                    adjacent = None;
                }
            }
            if let Some(pos) = adjacent {
                if let Some(other_num) = adjacents.remove(&pos) {
                    sum += num * other_num;
                } else {
                    adjacents.insert(pos, num);
                }
            }
            num = 0;
            adjacent = None;
        }
        println!("Part 2: {sum}");
    }
}
