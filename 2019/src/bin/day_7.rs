use itertools::Itertools;

use util::PerfTimer;

extern crate util;

#[derive(Clone)]
struct State {
    mem: Vec<isize>,
    pos: usize,
}

enum Code {
    Success(State),
    Stop(State),
}

fn get_args<const C: usize>(opcode: isize, mem: &[isize], params: &[isize]) -> [isize; C] {
    let mut place = 100;
    let mut args = [0; C];
    for (i, param) in params.iter().copied().enumerate() {
        let mode = (opcode / place) % 10;
        args[i] = match mode {
            0 => mem[param as usize],
            1 => param,
            _ => panic!("Invalid parameter mode"),
        };
        place *= 10;
    }
    args
}

fn execute_operation(
    State { mut mem, mut pos }: State,
    input: &mut impl FnMut() -> isize,
    output: &mut impl FnMut(isize),
) -> Code {
    match mem[pos] % 100 {
        1 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2]);
            mem[params[2] as usize] = arg_1 + arg_2;
            pos += 4;
            Code::Success(State { mem, pos })
        }
        2 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2]);
            mem[params[2] as usize] = arg_1 * arg_2;
            pos += 4;
            Code::Success(State { mem, pos })
        }
        3 => {
            let arg = mem[pos + 1];
            mem[arg as usize] = input();
            pos += 2;
            Code::Success(State { mem, pos })
        }
        4 => {
            let param = mem[pos + 1];
            let [arg] = get_args(mem[pos], &mem, &[param]);
            output(arg);
            pos += 2;
            Code::Success(State { mem, pos })
        }
        5 => {
            let params = mem[pos + 1..=pos + 2].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params);
            if arg_1 != 0 {
                pos = arg_2 as usize;
            } else {
                pos += 3;
            }
            Code::Success(State { mem, pos })
        }
        6 => {
            let params = mem[pos + 1..=pos + 2].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params);
            if arg_1 == 0 {
                pos = arg_2 as usize;
            } else {
                pos += 3;
            }
            Code::Success(State { mem, pos })
        }
        7 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2]);
            mem[params[2] as usize] = isize::from(arg_1 < arg_2);
            pos += 4;
            Code::Success(State { mem, pos })
        }
        8 => {
            let params = mem[pos + 1..=pos + 3].to_owned();
            let [arg_1, arg_2] = get_args(mem[pos], &mem, &params[..2]);
            mem[params[2] as usize] = isize::from(arg_1 == arg_2);
            pos += 4;
            Code::Success(State { mem, pos })
        }
        99 => Code::Stop(State { mem, pos }),
        op => panic!("Unrecognised opcode {}", op),
    }
}

fn run_program(mem: Vec<isize>, mut input: Vec<isize>) -> isize {
    input.reverse();
    let mut last_result = Code::Success(State { mem, pos: 0 });

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

fn run_one_in_one_out(state: &mut State, input: isize) -> Option<isize> {
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

fn run_one_in(state: &mut State, input: isize) -> bool {
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

fn run_configuration(mem: Vec<isize>, settings: &[isize]) -> isize {
    let mut signal = 0;
    for &setting in settings.iter() {
        signal = run_program(mem.clone(), vec![setting, signal]);
    }
    signal
}

fn main() {
    let input: Vec<isize> = util::get_day_input(7)
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut part_1 = isize::MIN;
        for config in (0..=4).permutations(5) {
            let result = run_configuration(input.clone(), &config);
            if result >= part_1 {
                part_1 = result;
            }
        }

        println!("Max signal: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut part_2 = 0;
        for config in (5..=9).permutations(5) {
            let mut current = 0;
            let mut state: Vec<State> = (0..=4)
                .map(|_| State {
                    mem: input.clone(),
                    pos: 0,
                })
                .collect();
            for (k, v) in config.iter().copied().enumerate() {
                assert!(run_one_in(&mut state[k], v))
            }
            'a: loop {
                for s in state.iter_mut() {
                    if let Some(c) = run_one_in_one_out(s, current) {
                        current = c;
                    } else {
                        break 'a;
                    }
                }
            }
            part_2 = part_2.max(current);
        }
        println!("Part 2: {part_2}");
    }
}
