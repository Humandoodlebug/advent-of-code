use regex::Regex;
use util::PerfTimer;

type Point = (isize, isize, isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

fn do_step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        let mut moon = moons[i];
        for m in &*moons {
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

    for moon in moons {
        moon.position.0 += moon.velocity.0;
        moon.position.1 += moon.velocity.1;
        moon.position.2 += moon.velocity.2;
    }
}

fn hcf(m: u128, n: u128) -> u128 {
    assert!(m > 0);
    assert!(n > 0);
    let mut hcf = None;
    for i in (1..=(m.min(n))).rev() {
        if m % i == 0 && n % i == 0 {
            hcf = Some(i);
            break;
        }
    }
    hcf.unwrap()
}

fn lcm(m: u128, n: u128) -> u128 {
    assert!(m > 0);
    assert!(n > 0);
    (m * n) / hcf(m, n)
}

fn main() {
    let input = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut moons = input.clone();
        for _step in 0..1000 {
            do_step(&mut moons);
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

    {
        let _timer = PerfTimer::new("Part 2");
        let mut moons = input.clone();
        let mut steps = (None, None, None);
        let mut step = 0;
        while steps.0.is_none() || steps.1.is_none() || steps.2.is_none() {
            do_step(&mut moons);
            step += 1;
            if steps.0.is_none()
                && moons.iter().enumerate().all(|(i, m)| {
                    m.position.0 == input[i].position.0 && m.velocity.0 == input[i].velocity.0
                })
            {
                steps.0 = Some(step);
            }
            if steps.1.is_none()
                && moons.iter().enumerate().all(|(i, m)| {
                    m.position.1 == input[i].position.1 && m.velocity.1 == input[i].velocity.1
                })
            {
                steps.1 = Some(step);
            }
            if steps.2.is_none()
                && moons.iter().enumerate().all(|(i, m)| {
                    m.position.2 == input[i].position.2 && m.velocity.2 == input[i].velocity.2
                })
            {
                steps.2 = Some(step);
            }
        }
        let part_2 = lcm(lcm(steps.0.unwrap(), steps.1.unwrap()), steps.2.unwrap());
        println!("Part 2: {part_2}");
    }
}
