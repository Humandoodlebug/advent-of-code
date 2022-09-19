use util::PerfTimer;

extern crate util;

struct State {
    mem: Vec<usize>,
    pos: usize,
}

enum Code {
    Success(State),
    Stop(State),
}

fn execute_operation(State { mut mem, mut pos }: State) -> Code {
    match mem[pos] {
        1 => {
            let params = (&mem[pos + 1..=pos + 3]).to_owned();
            mem[params[2]] = mem[params[0]] + mem[params[1]];
            pos += 4;
            Code::Success(State { mem, pos })
        }
        2 => {
            let args = (&mem[pos + 1..=pos + 3]).to_owned();
            mem[args[2]] = mem[args[0]] * mem[args[1]];
            pos += 4;
            Code::Success(State { mem, pos })
        }
        99 => Code::Stop(State { mem, pos }),
        op => panic!("Unrecognised opcode {}", op),
    }
}

fn run_program(mem: Vec<usize>) -> usize {
    let mut last_result = Code::Success(State { mem, pos: 0 });

    let final_state;

    loop {
        match last_result {
            Code::Success(state) => last_result = execute_operation(state),
            Code::Stop(state) => {
                final_state = state;
                break;
            }
        }
    }
    final_state.mem[0]
}

fn main() {
    let input: Vec<usize> = util::get_day_input(2)
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut mem = input.clone();
        mem[1] = 12;
        mem[2] = 2;
        let part1 = run_program(mem);

        println!("Part 1: {part1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        for noun in 0..100 {
            for verb in 0..100 {
                let mut mem = input.clone();
                mem[1] = noun;
                mem[2] = verb;
                let result = run_program(mem);
                if result == 19690720 {
                    println!("Part 2: {}", 100 * noun + verb);
                    return;
                }
            }
        }
    }
}
