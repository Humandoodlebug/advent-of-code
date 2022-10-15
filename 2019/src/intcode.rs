#[derive(Clone)]
pub struct State {
    pub mem: Vec<i128>,
    pub pos: usize,
    pub relative_base: i128,
}

pub enum Code {
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

impl State {
    pub fn execute_operation(
        self,
        mut input: impl FnMut() -> i128,
        mut output: impl FnMut(i128),
    ) -> Code {
        let State {
            mut mem,
            mut pos,
            mut relative_base,
        } = self;
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
}

pub fn run_to_completion(
    mem: Vec<i128>,
    mut input: impl FnMut() -> i128,
    mut output: impl FnMut(i128),
) -> State {
    let mut last_result = Code::Success(State {
        mem,
        pos: 0,
        relative_base: 0,
    });

    loop {
        match last_result {
            Code::Success(state) => last_result = state.execute_operation(&mut input, &mut output),
            Code::Stop(state) => break state,
        }
    }
}
