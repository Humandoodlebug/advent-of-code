use regex::Regex;
use util::PerfTimer;

type Point = (isize, isize, isize);

#[derive(Clone, Copy, Debug)]
struct Moon {
    position: Point,
    velocity: Point,
}

fn input() -> Vec<Moon> {
    let raw = util::get_day_input(12);
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut moons = Vec::new();
    for line in raw.trim().lines() {
        let caps = re.captures(line).unwrap();
        moons.push(Moon {
            position: (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
            ),
            velocity: (0, 0, 0),
        });
    }
    moons
}

fn main() {
    let moons = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut moons = moons.clone();
        for _ in 0..1000 {
            for i in 0..moons.len() {
                let mut moon = moons[i];
                for m in &moons {
                    match moon.position.0.cmp(&m.position.0) {
                        std::cmp::Ordering::Less => moon.velocity.0 += 1,
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => moon.velocity.0 -= 1,
                    }
                    match moon.position.1.cmp(&m.position.1) {
                        std::cmp::Ordering::Less => moon.velocity.1 += 1,
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => moon.velocity.1 -= 1,
                    }
                    match moon.position.2.cmp(&m.position.2) {
                        std::cmp::Ordering::Less => moon.velocity.2 += 1,
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => moon.velocity.2 -= 1,
                    }
                }
                moons[i] = moon;
            }

            for moon in &mut moons {
                moon.position.0 += moon.velocity.0;
                moon.position.1 += moon.velocity.1;
                moon.position.2 += moon.velocity.2;
            }
        }
        let part_1: isize = moons
            .into_iter()
            .map(
                |Moon {
                     position: (p_x, p_y, p_z),
                     velocity: (v_x, v_y, v_z),
                 }| {
                    (p_x.abs() + p_y.abs() + p_z.abs()) * (v_x.abs() + v_y.abs() + v_z.abs())
                },
            )
            .sum();
        println!("Part 1: {part_1}");
    }
}
