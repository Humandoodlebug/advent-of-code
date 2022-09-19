use std::collections::HashMap;

use util::get_day_input;
use Argument::*;
use Instruction::*;

#[derive(Clone, Debug)]
struct State<'a> {
    inp: &'a [isize],
    vars: HashMap<char, isize>,
}

impl<'a> State<'a> {
    fn new(inp: &'a [isize]) -> Self {
        Self {
            inp,
            vars: HashMap::from([('w', 0), ('x', 0), ('y', 0), ('z', 0)]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Argument {
    Literal(isize),
    Var(char),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Inp(char),
    Add(char, Argument),
    Mul(char, Argument),
    Div(char, Argument),
    Mod(char, Argument),
    Eql(char, Argument),
}

fn eval(argument: Argument, state: &State) -> isize {
    match argument {
        Literal(x) => x,
        Var(var) => state.vars[&var],
    }
}

fn parse_arg(arg: &str) -> Argument {
    if ('w'..='z').contains(&arg.chars().next().unwrap()) {
        Var(arg.chars().next().unwrap())
    } else {
        Literal(arg.parse().unwrap())
    }
}

fn input() -> Vec<Instruction> {
    get_day_input(24)
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let var = parts[1].chars().next().unwrap();
            match parts[0] {
                "inp" => Inp(var),
                "add" => Add(var, parse_arg(parts[2])),
                "mul" => Mul(var, parse_arg(parts[2])),
                "div" => Div(var, parse_arg(parts[2])),
                "mod" => Mod(var, parse_arg(parts[2])),
                "eql" => Eql(var, parse_arg(parts[2])),
                _ => panic!(),
            }
        })
        .collect()
}

fn exec_instruction(instruction: Instruction, mut state: State) -> State {
    match instruction {
        Inp(var) => {
            // println!("z: {}", state.vars[&'z']);
            *state.vars.get_mut(&var).unwrap() = state.inp[0];
            state.inp = &state.inp[1..];
        }
        Add(var, arg) => *state.vars.get_mut(&var).unwrap() += eval(arg, &state),
        Mul(var, arg) => *state.vars.get_mut(&var).unwrap() *= eval(arg, &state),
        Div(var, arg) => *state.vars.get_mut(&var).unwrap() /= eval(arg, &state),
        Mod(var, arg) => *state.vars.get_mut(&var).unwrap() %= eval(arg, &state),
        Eql(var, arg) => {
            *state.vars.get_mut(&var).unwrap() = isize::from(state.vars[&var] == eval(arg, &state))
        }
    }
    state
}

fn main() {
    let instructions = input();

    // let model_number = 91599994399395isize;  // Part 1
    let model_number = 71111591176151isize; // Part 2
    let input = model_number.to_string();
    // if input.chars().contains(&'0') {
    //     continue;
    // }
    let input: Vec<isize> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let mut state = State::new(&input);
    for &instruction in &instructions {
        state = exec_instruction(instruction, state)
    }

    println!("z = {}", state.vars[&'z']);
}

// Pseudocode:

/*
x_mods <- [13, 15, 15, 11, -16, -11, -6, 11, 10, -10, -8, -11, 12, -15]
y_mods <- [5, 14, 15, 16, 8, 9, 2, 13, 16, 6, 6, 9, 11, 5]
digits <- input()
w,x,y,z <- 0
for i in 0..len(digits)
    w <- digits[i]
    x <- z % 26 + x_mods[i]
    if x_mods[i] < 0
        z <- z / 26
    if x != w
        z <- z * 26
        y <- (w + y_mods[i]) * x
        z <- z + y

*/
// See spreadsheet (Day 24.ods) for solving process
