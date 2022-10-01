use util::PerfTimer;

extern crate util;

#[derive(Clone)]
struct State {
    mem: Vec<i128>,
    pos: usize,
    relative_base: i128,
}

enum Code {
    Success(State),
    Stop(State),
}

fn get_args<const C: usize>(
    opcode: i128,
    mem: &[i128],
    params: &[i128],
    relative_base: i128,
) -> [i128; C] {
    let mut place = 100;
    let mut args = [0; C];
    for (i, param) in params.iter().copied().enumerate() {
        let mode = (opcode / place) % 10;
        args[i] = match mode {
            0 => mem[param as usize],
            1 => param,
            2 => mem[(relative_base + param) as usize],
            _ => panic!("Invalid parameter mode"),
        };
        place *= 10;
    }
    args
}

fn get_addr(opcode: i128, params: &[i128], relative_base: i128, param_index: usize) -> usize {
    let mut place = 100;
    for _ in 0..param_index {
        place *= 10;
    }
    let mode = (opcode / place) % 10;
    match mode {
        0 => params[param_index] as usize,
        1 => panic!("Immediate mode is invalid here"),
        2 => (relative_base + params[param_index]) as usize,
        _ => panic!("Invalid parameter mode"),
    }
}

fn execute_operation(
    State {
        mut mem,
        mut pos,
        mut relative_base,
    }: State,
    input: &mut impl FnMut() -> i128,
    output: &mut impl FnMut(i128),
) -> Code {
    match mem[pos] % 100 {
        1 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2], relative_base);
            let output_addr = get_addr(mem[pos], &params, relative_base, 2);
            mem[output_addr] = arg_1 + arg_2;
            pos += 4;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        2 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2], relative_base);
            let output_addr = get_addr(mem[pos], &params, relative_base, 2);
            mem[output_addr] = arg_1 * arg_2;
            pos += 4;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        3 => {
            let param = mem[pos + 1];
            let output_addr = get_addr(mem[pos], &[param], relative_base, 0);
            mem[output_addr] = input();
            pos += 2;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        4 => {
            let param = mem[pos + 1];
            let [arg] = get_args(mem[pos], &mem, &[param], relative_base);
            output(arg);
            pos += 2;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        5 => {
            let params = mem[pos + 1..=pos + 2].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params, relative_base);
            if arg_1 != 0 {
                pos = arg_2 as usize;
            } else {
                pos += 3;
            }
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        6 => {
            let params = mem[pos + 1..=pos + 2].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params, relative_base);
            if arg_1 == 0 {
                pos = arg_2 as usize;
            } else {
                pos += 3;
            }
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        7 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2], relative_base);
            let output_addr = get_addr(mem[pos], &params, relative_base, 2);
            mem[output_addr] = i128::from(arg_1 < arg_2);
            pos += 4;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        8 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2], relative_base);
            let output_addr = get_addr(mem[pos], &params, relative_base, 2);
            mem[output_addr] = i128::from(arg_1 == arg_2);
            pos += 4;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        9 => {
            let param = mem[pos + 1];
            let [arg] = get_args(mem[pos], &mem, &[param], relative_base);
            relative_base += arg;
            pos += 2;
            Code::Success(State {
                mem,
                pos,
                relative_base,
            })
        }
        99 => Code::Stop(State {
            mem,
            pos,
            relative_base,
        }),
        op => panic!("Unrecognised opcode {}", op),
    }
}

fn run_program(mem: Vec<i128>, mut input: Vec<i128>) -> i128 {
    input.reverse();
    let mut last_result = Code::Success(State {
        mem,
        pos: 0,
        relative_base: 0,
    });

    let mut output = Vec::new();

    loop {
        match last_result {
            Code::Success(state) => {
                last_result =
                    execute_operation(state, &mut || input.pop().unwrap(), &mut |x| output.push(x))
            }
            Code::Stop(_state) => {
                for &i in &output[..output.len() - 1] {
                    assert_eq!(i, 0);
                }
                return *output.last().unwrap();
            }
        }
    }
}

fn run_one_in_one_out(state: &mut State, input: i128) -> Option<i128> {
    let mut input = Some(input);
    let mut output = None;
    let mut last_result = Code::Success(state.clone());

    loop {
        match last_result {
            Code::Success(s) => {
                if output.is_some() {
                    *state = s;
                    break output;
                }
                last_result =
                    execute_operation(s, &mut || input.take().unwrap(), &mut |x| output = Some(x))
            }
            Code::Stop(s) => {
                *state = s;
                break output;
            }
        }
    }
}

fn run_one_in(state: &mut State, input: i128) -> bool {
    let mut input = Some(input);
    let mut last_result = Code::Success(state.clone());

    loop {
        match last_result {
            Code::Success(s) => {
                if input.is_none() {
                    *state = s;
                    break true;
                }
                last_result = execute_operation(s, &mut || input.take().unwrap(), &mut |_| {
                    panic!("Received unexpected output")
                })
            }
            Code::Stop(s) => {
                *state = s;
                break false;
            }
        }
    }
}

fn main() {
    let input: Vec<i128> = util::get_day_input(9)
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut mem = input.clone();
        mem.extend((0..1000000).map(|_| 0));
        let part_1 = run_program(mem, vec![1]);
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut mem = input;
        mem.extend((0..1000000).map(|_| 0));
        let part_2 = run_program(mem, vec![2]);
        println!("Part 2: {part_2}");
    }
}
