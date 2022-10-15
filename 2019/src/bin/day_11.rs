use std::{cell::RefCell, collections::HashMap, rc::Rc};

use util::PerfTimer;

#[derive(Clone, Copy, Debug)]
enum Paint {
    White,
    Black,
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Turn {
    Left,
    Right,
}

#[derive(Clone)]
struct Hull {
    panels: HashMap<(isize, isize), Paint>,
    robot_pos: (isize, isize),
    robot_direction: Direction,
}

impl Hull {
    fn new() -> Self {
        Self {
            panels: HashMap::new(),
            robot_pos: (0, 0),
            robot_direction: Direction::Up,
        }
    }

    fn inspect(&self) -> Paint {
        self.panels
            .get(&self.robot_pos)
            .copied()
            .unwrap_or(Paint::Black)
    }

    fn paint(&mut self, paint: Paint) {
        self.panels.insert(self.robot_pos, paint);
    }
    fn mv(&mut self, turn: Turn) {
        self.robot_direction = match turn {
            Turn::Left => match self.robot_direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            Turn::Right => match self.robot_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        };
        match self.robot_direction {
            Direction::Up => self.robot_pos.1 -= 1,
            Direction::Right => self.robot_pos.0 += 1,
            Direction::Down => self.robot_pos.1 += 1,
            Direction::Left => self.robot_pos.0 -= 1,
        }
    }
}

fn paint_hull(mut mem: Vec<i128>, hull: Hull) -> Hull {
    mem.extend((0..1000000).map(|_| 0));

    let hull = Rc::new(RefCell::new(hull));
    let inp_hull = Rc::clone(&hull);
    let out_hull = Rc::clone(&hull);
    let mut should_paint = true;
    util::intcode::run_to_completion(
        mem,
        move || match inp_hull.borrow().inspect() {
            Paint::Black => 0,
            Paint::White => 1,
        },
        move |x| {
            if should_paint {
                out_hull.borrow_mut().paint(match x {
                    0 => Paint::Black,
                    1 => Paint::White,
                    x => panic!("Unrecognised paint colour id {x}"),
                });
            } else {
                out_hull.borrow_mut().mv(match x {
                    0 => Turn::Left,
                    1 => Turn::Right,
                    x => panic!("Unrecognised paint colour id {x}"),
                });
            }
            should_paint = !should_paint;
        },
    );

    let hull = hull.borrow().clone();
    hull
}

fn main() {
    let input: Vec<i128> = util::get_day_input(11)
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    {
        let _timer = PerfTimer::new("Part 1");
        let hull = paint_hull(input.clone(), Hull::new());
        let part_1 = hull.panels.len();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut hull = Hull::new();
        hull.panels.insert((0, 0), Paint::White);
        let hull = paint_hull(input, hull);
        let max_x = *hull.panels.keys().map(|(x, _y)| x).max().unwrap();
        let max_y = *hull.panels.keys().map(|(_x, y)| y).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                match hull.panels.get(&(x, y)).copied().unwrap_or(Paint::Black) {
                    Paint::White => print!("#"),
                    Paint::Black => print!("."),
                }
            }
            println!();
        }
    }
}
