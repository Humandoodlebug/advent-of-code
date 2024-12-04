use util::PerfTimer;

fn input() -> Vec<Vec<char>> {
    let input = util::get_day_input(4);
    input.lines().map(|r| r.chars().collect()).collect()
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn search(grid: &[Vec<char>], mv: (i32, i32)) -> usize {
    let mut count = 0;
    for y in 0..(grid.len() as i32) {
        for x in 0..(grid[0].len() as i32) {
            let x_max = x + mv.0 * 3;
            let y_max = y + mv.1 * 3;
            if x_max < 0 || x_max >= grid[0].len() as i32 || y_max < 0 || y_max >= grid.len() as i32
            {
                continue;
            }

            if grid[y as usize][x as usize] == 'X'
                && grid[(y + mv.1) as usize][(x + mv.0) as usize] == 'M'
                && grid[(y + mv.1 * 2) as usize][(x + mv.0 * 2) as usize] == 'A'
                && grid[(y + mv.1 * 3) as usize][(x + mv.0 * 3) as usize] == 'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let grid = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = search(&grid, (1, 0))
            + search(&grid, (0, 1))
            + search(&grid, (1, 1))
            + search(&grid, (-1, 0))
            + search(&grid, (0, -1))
            + search(&grid, (-1, -1))
            + search(&grid, (1, -1))
            + search(&grid, (-1, 1));

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut count = 0;
        for y in 0..grid.len() - 2 {
            for x in 0..grid[0].len() - 2 {
                if grid[y + 1][x + 1] == 'A'
                    && ((grid[y][x] == 'M' && grid[y + 2][x + 2] == 'S')
                        || (grid[y][x] == 'S' && grid[y + 2][x + 2] == 'M'))
                    && ((grid[y][x + 2] == 'M' && grid[y + 2][x] == 'S')
                        || (grid[y][x + 2] == 'S' && grid[y + 2][x] == 'M'))
                {
                    count += 1;
                }
            }
        }

        println!("Part 2: {count}");
    }
}
