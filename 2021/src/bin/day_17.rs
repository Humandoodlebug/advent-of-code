extern crate util;

fn input() -> ((i32, i32), (i32, i32)) {
    ((244, 303), (-91, -54))
}

fn main() {
    let ((x_min, x_max), (y_min, y_max)) = input();

    let x_vel = (1..).find(|x| {
        let p = (x * (x + 1)) / 2;
        p >= x_min && p <= x_max
    }).unwrap();
    let y_vel = - y_min - 1;
    dbg!((x_vel, y_vel));
    let part1 = (y_vel * (y_vel + 1)) / 2;
    println!("Part 1: {}", part1);

    let x_vel_min = x_vel;
    let x_vel_max = x_max;
    let y_vel_min = y_min;
    let y_vel_max = y_vel;
    let mut part2 = 0;
    for x in x_vel_min..=x_vel_max {
        for y in y_vel_min..=y_vel_max {
            let (mut x_vel, mut y_vel) = (x, y);
            let (mut x_pos, mut y_pos) = (0, 0);
            while x_pos <= x_max && y_pos >= y_min {
                x_pos += x_vel;
                y_pos += y_vel;
                if x_pos >= x_min && x_pos <= x_max && y_pos >= y_min && y_pos <= y_max {
                    part2 += 1;
                    break;
                }
                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
        }
    }
    println!("Part 2: {}", part2);
}
