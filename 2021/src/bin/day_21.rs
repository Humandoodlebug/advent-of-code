use util::PerfTimer;

extern crate util;

fn input() -> (i32, i32) {
    let raw = util::get_day_input(21);
    let mut lines = raw.lines();
    let p1_start = lines.next().unwrap()[28..].parse().unwrap();
    let p2_start = lines.next().unwrap()[28..].parse().unwrap();
    (p1_start, p2_start)
}

fn roll(dice: &mut i32) -> i32 {
    *dice += 1;
    *dice
}

fn get_move(dice: &mut i32) -> i32 {
    let r1 = roll(dice);
    let r2 = roll(dice);
    let r3 = roll(dice);
    r1 + r2 + r3
}

fn play_part_2(
    dice: &[(i32, u128)],
    p1_pos: i32,
    p2_pos: i32,
    p1_score: i32,
    p2_score: i32,
) -> (u128, u128) {
    if p2_score >= 21 {
        return (0, 1);
    }
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for &(roll, frequency) in dice {
        let (mut new_p1_pos, new_p1_score);
        new_p1_pos = p1_pos + roll;
        if new_p1_pos > 10 {
            new_p1_pos -= 10;
        }
        new_p1_score = p1_score + new_p1_pos;

        let (p2, p1) = play_part_2(dice, p2_pos, new_p1_pos, p2_score, new_p1_score);
        p1_wins += p1 * frequency;
        p2_wins += p2 * frequency;
    }
    (p1_wins, p2_wins)
}

fn main() {
    let (p1_start, p2_start) = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut p1_pos = p1_start;
        let mut p2_pos = p2_start;
        let mut p1_score = 0;
        let mut p2_score = 0;
        let mut dice = 0;

        loop {
            let p1_move = get_move(&mut dice);
            p1_pos += p1_move;
            while p1_pos > 10 {
                p1_pos -= 10;
            }
            p1_score += p1_pos;

            if p1_score >= 1000 {
                let part1 = p2_score * dice;
                println!("Part 1: {part1}");
                break;
            }

            let p2_move = get_move(&mut dice);
            p2_pos += p2_move;
            while p2_pos > 10 {
                p2_pos -= 10;
            }
            p2_score += p2_pos;

            if p2_score >= 1000 {
                let part1 = p1_score * dice;
                println!("Part 1: {part1}");
                break;
            }
        }
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let dirac_dice = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let (part2_p1, part2_p2) = play_part_2(&dirac_dice, p1_start, p2_start, 0, 0);
        let part2 = std::cmp::max(part2_p1, part2_p2);
        println!("Part 2: {part2}");
    }
}
