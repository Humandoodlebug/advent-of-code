use util::PerfTimer;

fn input() -> Vec<Vec<i8>> {
    util::get_day_input(8)
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - b'0') as i8).collect())
        .collect()
}

fn generate_visibility_matrix(grid: &[Vec<i8>]) -> Vec<Vec<bool>> {
    let mut results = vec![vec![false; grid[0].len()]; grid.len()];

    let mut outer_max_heights = vec![-1; grid[0].len()];
    for (i, line) in grid.iter().enumerate() {
        let mut inner_max_height = -1;
        for (j, &h) in line.iter().enumerate() {
            if h > inner_max_height {
                results[i][j] = true;
                inner_max_height = h;
            }
            if h > outer_max_heights[j] {
                results[i][j] = true;
                outer_max_heights[j] = h;
            }
        }
    }

    let mut outer_max_heights = vec![-1; grid[0].len()];
    for (i, line) in grid.iter().enumerate().rev() {
        let mut inner_max_height = -1;
        for (j, &h) in line.iter().enumerate().rev() {
            if h > inner_max_height {
                results[i][j] = true;
                inner_max_height = h;
            }
            if h > outer_max_heights[j] {
                results[i][j] = true;
                outer_max_heights[j] = h;
            }
        }
    }

    results
}

fn calculate_scenic_score(grid: &[Vec<i8>], tree_i: usize, tree_j: usize) -> usize {
    let tree_height = grid[tree_i][tree_j];
    let max_i = grid.len() - 1;
    let max_j = grid[0].len() - 1;

    let top = if tree_i == 0 {
        0
    } else {
        let mut i = tree_i - 1;
        while i > 0 && grid[i][tree_j] < tree_height {
            i -= 1;
        }
        tree_i - i
    };
    let bottom = if tree_i == max_i {
        0
    } else {
        let mut i = tree_i + 1;
        while i < max_i && grid[i][tree_j] < tree_height {
            i += 1;
        }
        i - tree_i
    };
    let left = if tree_j == 0 {
        0
    } else {
        let mut j = tree_j - 1;
        while j > 0 && grid[tree_i][j] < tree_height {
            j -= 1;
        }
        tree_j - j
    };
    let right = if tree_j == max_j {
        0
    } else {
        let mut j = tree_j + 1;
        while j < max_j && grid[tree_i][j] < tree_height {
            j += 1;
        }
        j - tree_j
    };
    top * bottom * left * right
}

fn main() {
    let grid = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let visibility_matrix = generate_visibility_matrix(&grid);
        let part_1: usize = visibility_matrix
            .iter()
            .map(|l| l.iter().filter(|&&x| x).count())
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = (0..grid.len())
            .map(|i| {
                (0..grid[0].len())
                    .map(|j| calculate_scenic_score(&grid, i, j))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        println!("Part 2: {part_2}");
    }
}
