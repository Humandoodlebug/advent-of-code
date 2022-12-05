use util::PerfTimer;

#[derive(Clone, Copy, Debug)]
struct Instruction {
    num_to_move: usize,
    from: usize,
    to: usize,
}

fn input() -> (Vec<Vec<char>>, Vec<Instruction>) {
    let raw = util::get_day_input(5);
    let (raw_stacks, raw_moves) = raw.trim_end().split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in raw_stacks.lines() {
        if !line.starts_with(" 1 ") {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }

    let moves = raw_moves
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            Instruction {
                num_to_move: words[1].parse().unwrap(),
                from: words[3].parse().unwrap(),
                to: words[5].parse().unwrap(),
            }
        })
        .collect();

    (stacks, moves)
}

fn main() {
    let (initial_state, instructions) = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: String = instructions
            .iter()
            .copied()
            .fold(initial_state.clone(), |mut state, instruction| {
                for _ in 0..instruction.num_to_move {
                    let temp = state[instruction.from - 1].pop().unwrap();
                    state[instruction.to - 1].push(temp);
                }
                state
            })
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut temp: Vec<char> = Vec::new();
        let part_2: String = instructions
            .iter()
            .copied()
            .fold(initial_state, |mut state, instruction| {
                assert!(temp.is_empty());
                for _ in 0..instruction.num_to_move {
                    temp.push(state[instruction.from - 1].pop().unwrap());
                }
                for _ in 0..instruction.num_to_move {
                    state[instruction.to - 1].push(temp.pop().unwrap());
                }
                state
            })
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect();
        println!("Part 2: {part_2}");
    }
}
