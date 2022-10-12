fn input() -> Vec<Vec<bool>> {
    let raw = util::get_day_input(10);
    let mut grid = Vec::new();
    for line in raw.trim().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        grid.push(row);
    }
    grid
}

fn is_line_of_sight(grid: &[Vec<bool>], pos_a: (usize, usize), pos_b: (usize, usize)) -> bool {
    let pos_b_from_a = (
        (pos_b.0 as isize) - (pos_a.0 as isize),
        (pos_b.1 as isize) - (pos_a.1 as isize),
    );
    for (asteroid_y, asteroid_row) in grid.iter().enumerate() {
        for (asteroid_x, &asteroid_cell) in asteroid_row.iter().enumerate() {
            if asteroid_cell
                && pos_a != (asteroid_x, asteroid_y)
                && pos_b != (asteroid_x, asteroid_y)
            {
                let asteroid_pos_from_a = (
                    (asteroid_x as isize) - (pos_a.0 as isize),
                    (asteroid_y as isize) - (pos_a.1 as isize),
                );
                if asteroid_pos_from_a.0 * pos_b_from_a.1 == asteroid_pos_from_a.1 * pos_b_from_a.0
                    && ((asteroid_pos_from_a.0 >= 0 && pos_b_from_a.0 >= 0)
                        || (asteroid_pos_from_a.0 <= 0 && pos_b_from_a.0 <= 0))
                    && ((asteroid_pos_from_a.1 >= 0 && pos_b_from_a.1 >= 0)
                        || (asteroid_pos_from_a.1 <= 0 && pos_b_from_a.1 <= 0))
                    && (asteroid_pos_from_a.0.abs() < pos_b_from_a.0.abs()
                        || asteroid_pos_from_a.1.abs() < pos_b_from_a.1.abs())
                {
                    return false;
                }
            }
        }
    }
    true
}

fn count_asteroids_visible_from(grid: &[Vec<bool>], pos: (usize, usize)) -> usize {
    let mut count = 0;
    for (asteroid_y, asteroid_row) in grid.iter().enumerate() {
        for (asteroid_x, &asteroid_cell) in asteroid_row.iter().enumerate() {
            if asteroid_cell
                && pos != (asteroid_x, asteroid_y)
                && is_line_of_sight(grid, pos, (asteroid_x, asteroid_y))
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let grid = input();
    let mut asteroids_in_view = 0;
    for (station_y, station_row) in grid.iter().enumerate() {
        for (station_x, &station_cell) in station_row.iter().enumerate() {
            if station_cell {
                let new_in_view = count_asteroids_visible_from(&grid, (station_x, station_y));
                asteroids_in_view = asteroids_in_view.max(new_in_view);
            }
        }
    }

    println!("Part 1: {asteroids_in_view}");
}
