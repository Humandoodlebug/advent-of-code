use util::PerfTimer;

#[derive(Clone, Copy)]
enum Instruction {
    NoOp,
    AddX(i64),
}

fn input() -> Vec<Instruction> {
    util::get_day_input(10)
        .trim()
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split(' ').collect();
            match line[0] {
                "noop" => Instruction::NoOp,
                "addx" => Instruction::AddX(line[1].parse().unwrap()),
                o => panic!("Unrecognised instruction {o:?}"),
            }
        })
        .collect()
}

fn cycle_v(cycle: i64, reg_x: i64) -> i64 {
    if (cycle - 20) % 40 == 0 {
        cycle * reg_x
    } else {
        0
    }
}

fn draw_pixel(pixels: &mut [[bool; 40]; 6], cycle: i64, reg_x: i64) {
    if reg_x.abs_diff((cycle - 1) % 40) < 2 {
        let pos_x = (cycle as usize - 1) % 40;
        let pos_y = (cycle as usize - 1) / 40;
        pixels[pos_y][pos_x] = true;
    }
}

fn main() {
    let instructions = input();
    let _timer = PerfTimer::new("Both parts");
    let mut cycle = 1;
    let mut part_1 = 0;
    let mut part_2 = [[false; 40]; 6];
    let mut reg_x = 1;
    for instruction in instructions.iter().copied() {
        match instruction {
            Instruction::NoOp => {
                part_1 += cycle_v(cycle, reg_x);
                draw_pixel(&mut part_2, cycle, reg_x);
                cycle += 1;
            },
            Instruction::AddX(v) => {
                part_1 += cycle_v(cycle, reg_x);
                draw_pixel(&mut part_2, cycle, reg_x);
                cycle += 1;
                part_1 += cycle_v(cycle, reg_x);
                draw_pixel(&mut part_2, cycle, reg_x);
                reg_x += v;
                cycle += 1;
            }
        }
    }
    println!("Part 1: {part_1}");
    println!("Part 2:");
    for line in part_2 {
        for pixel in line {
            if pixel {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

}
