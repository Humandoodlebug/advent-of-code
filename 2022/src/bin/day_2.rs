#[derive(Clone, Copy, Debug)]
enum Opponent {
    A,
    B,
    C,
}

#[derive(Clone, Copy, Debug)]
enum Player {
    X,
    Y,
    Z,
}

fn input() -> Vec<(Opponent, Player)> {
    util::get_day_input(2)
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let l = match l {
                "A" => Opponent::A,
                "B" => Opponent::B,
                "C" => Opponent::C,
                _ => panic!("Unrecognised opponent move {l:?}"),
            };
            let r = match r {
                "X" => Player::X,
                "Y" => Player::Y,
                "Z" => Player::Z,
                _ => panic!("Unrecognised player move {r:?}"),
            };
            (l, r)
        })
        .collect()
}

fn main() {
    let move_table = input();
    {
        let _timer = util::PerfTimer::new("Part 1");
        let part_1: u64 = move_table
            .iter()
            .copied()
            .map(|(opponent_move, player_move)| {
                let move_score = match player_move {
                    Player::X => 1,
                    Player::Y => 2,
                    Player::Z => 3,
                };
                let outcome_score = match (opponent_move, player_move) {
                    (Opponent::A, Player::X)
                    | (Opponent::B, Player::Y)
                    | (Opponent::C, Player::Z) => 3,
                    (Opponent::A, Player::Y)
                    | (Opponent::B, Player::Z)
                    | (Opponent::C, Player::X) => 6,
                    (Opponent::A, Player::Z)
                    | (Opponent::B, Player::X)
                    | (Opponent::C, Player::Y) => 0,
                };
                move_score + outcome_score
            })
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = util::PerfTimer::new("Part 2");
        #[allow(clippy::identity_op)]
        let part_2: u64 = move_table
            .iter()
            .copied()
            .map(
                |(opponent_move, player_move)| match (opponent_move, player_move) {
                    (Opponent::A, Player::X) => 0 + 3,
                    (Opponent::B, Player::Y) => 3 + 2,
                    (Opponent::C, Player::Z) => 6 + 1,
                    (Opponent::A, Player::Y) => 3 + 1,
                    (Opponent::B, Player::Z) => 6 + 3,
                    (Opponent::C, Player::X) => 0 + 2,
                    (Opponent::A, Player::Z) => 6 + 2,
                    (Opponent::B, Player::X) => 0 + 1,
                    (Opponent::C, Player::Y) => 3 + 3,
                },
            )
            .sum();
        println!("Part 2: {part_2}");
    }
}
