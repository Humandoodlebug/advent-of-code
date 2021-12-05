extern crate util;

fn input() -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let lines: Vec<String> = util::get_day_input(4).lines().map(str::to_owned).collect();

    let calls: Vec<i32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    let grid_lines = &lines[2..];
    let grid_chunks = grid_lines.split(|s| s.is_empty());
    let grids: Vec<Vec<Vec<i32>>> = grid_chunks
        .map(|ls| {
            ls.iter()
                .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
                .collect()
        })
        .collect();

    (calls, grids)
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<i32>>,
    calls: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        let calls = vec![vec![false; grid[0].len()]; grid.len()];
        Grid { grid, calls }
    }

    fn calc_win(&self, x: i32) -> i32 {
        let mut sum = 0;
        for i in 0..self.calls.len() {
            for j in 0..self.calls[0].len() {
                if !self.calls[i][j] {
                    sum += self.grid[i][j];
                }
            }
        }
        sum * x
    }

    pub fn call(&mut self, x: i32) -> Option<i32> {
        let mut wins: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == x {
                    wins.push((i, j));
                    self.calls[i][j] = true;
                }
            }
        }

        for (i, j) in wins {
            if self.calls[i].iter().all(|x| *x) | self.calls.iter().map(|l| l[j]).all(|x| x) {
                return Some(self.calc_win(self.grid[i][j]));
            }
        }

        None
    }
}

fn part_1(calls: &[i32], mut grids: Vec<Grid>) -> i32 {
    for &call in calls {
        for grid in grids.iter_mut() {
            if let Some(score) = grid.call(call) {
                return score;
            }
        }
    }
    panic!("Nobody won!!!");
}

fn part_2(calls: &[i32], mut grids: Vec<Grid>) -> i32 {
    let mut winners = vec![false; grids.len()];
    let mut losing_score = -1;

    for &call in calls {
        for (i, grid) in grids.iter_mut().enumerate() {
            if winners[i] {
                continue;
            }
            if let Some(score) = grid.call(call) {
                winners[i] = true;
                losing_score = score;
            }
        }
    }
    losing_score
}

fn main() {
    let (calls, grids) = input();
    let grids: Vec<Grid> = grids.iter().map(|s| Grid::new(s.clone())).collect();

    let p1 = part_1(&calls, grids.clone());
    println!("Part 1: {}", p1);

    let p2 = part_2(&calls, grids);
    println!("Part 2: {}", p2);
}
