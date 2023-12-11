use util::PerfTimer;

fn input() -> Vec<Vec<bool>> {
    util::get_day_input(11)
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn main() {
    let map = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut map = map.clone();
        for row in (0..map.len()).rev() {
            if map[row].iter().all(|&x| !x) {
                map.insert(row, vec![false; map[0].len()]);
            }
        }
        for col in (0..map[0].len()).rev() {
            if map.iter().all(|row| !row[col]) {
                for row in &mut map {
                    row.insert(col, false);
                }
            }
        }
        let galaxies: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, x)| **x)
                    .map(move |(col, _)| (row, col))
            })
            .collect();

        let mut sum = 0;
        for g1 in &galaxies {
            for g2 in &galaxies {
                if g1 == g2 {
                    continue;
                }
                sum += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)
            }
        }
        let part_1 = sum / 2;
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        const EMPTY_SIZE: usize = 1_000_000;

        let empty_rows: Vec<usize> = map
            .iter()
            .enumerate()
            .filter(|(_row, line)| line.iter().all(|x| !*x))
            .map(|(row, _)| row)
            .collect();

        let empty_cols: Vec<usize> = (0..map[0].len())
            .filter(|col| map.iter().all(|row| !row[*col]))
            .collect();

        let galaxies: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, x)| **x)
                    .map(move |(col, _)| (row, col))
            })
            .collect();

        let mut sum = 0;
        for g1 in &galaxies {
            for g2 in &galaxies {
                if g1 == g2 {
                    continue;
                }
                let min_row = g1.0.min(g2.0);
                let min_col = g1.1.min(g2.1);
                let max_row = g1.0.max(g2.0);
                let max_col = g1.1.max(g2.1);

                let empty_rows_i = empty_rows
                    .iter()
                    .filter(|&&row| row > min_row && row < max_row)
                    .count();

                let empty_cols_i = empty_cols
                    .iter()
                    .filter(|&&col| col > min_col && col < max_col)
                    .count();

                let total = g1.0.abs_diff(g2.0)
                    + g1.1.abs_diff(g2.1)
                    + (empty_rows_i + empty_cols_i) * (EMPTY_SIZE - 1);

                sum += total;
            }
        }
        let part_2 = sum / 2;
        println!("Part 2: {part_2}");
    }
}
