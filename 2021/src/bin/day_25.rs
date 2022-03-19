use util::get_day_input;
use Position::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Position {
    Empty,
    EastCucumber,
    SouthCucumber,
}

type Grid = Vec<Vec<Position>>;

fn input() -> Grid {
    let raw = get_day_input(25);
    let width = raw.lines().next().unwrap().len();
    let height = raw.lines().count();
    let mut grid = vec![vec![Empty; height]; width];
    for (y, line) in raw.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x][y] = match c {
                '.' => Empty,
                '>' => EastCucumber,
                'v' => SouthCucumber,
                _ => panic!(),
            }
        }
    }
    grid
}

fn print_grid(grid: &Grid) {
    for y in 0..grid[0].len() {
        let mut line = String::new();
        #[allow(clippy::needless_range_loop)]
        for x in 0..grid.len() {
            line += match grid[x][y] {
                Empty => ".",
                EastCucumber => ">",
                SouthCucumber => "v",
            }
        }
        println!("{}", line);
    }
}

fn generate_next_state(grid: &Grid) -> (Grid, bool) {
    let mut moved = false;
    let mut new_grid = vec![vec![Empty; grid[0].len()]; grid.len()];

    for (x, column) in grid.iter().enumerate() {
        for (y, p) in column.iter().enumerate() {
            if *p == EastCucumber {
                let new_x = (x + 1) % grid.len();
                if grid[new_x][y] == Empty {
                    new_grid[new_x][y] = EastCucumber;
                    moved = true;
                } else {
                    new_grid[x][y] = EastCucumber;
                }
            } else if *p == SouthCucumber {
                new_grid[x][y] = SouthCucumber;
            }
        }
    }

    let grid = new_grid;
    let mut new_grid = vec![vec![Empty; grid[0].len()]; grid.len()];

    for (x, column) in grid.iter().enumerate() {
        for (y, p) in column.iter().enumerate() {
            if *p == SouthCucumber {
                let new_y = (y + 1) % grid[0].len();
                if grid[x][new_y] == Empty {
                    new_grid[x][new_y] = SouthCucumber;
                    moved = true;
                } else {
                    new_grid[x][y] = SouthCucumber;
                }
            } else if *p == EastCucumber {
                new_grid[x][y] = EastCucumber;
            }
        }
    }

    (new_grid, moved)
}

fn main() {
    let mut grid = input();
    let mut step = 0;

    loop {
        step += 1;
        let (g, changed) = generate_next_state(&grid);
        grid = g;
        if !changed {
            break;
        }
    }

    println!("\n\nFinal State {}:\n", step);
    print_grid(&grid);

    println!("\n\nStep: {}", step);
}
