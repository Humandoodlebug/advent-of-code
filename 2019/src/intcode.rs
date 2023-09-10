pub use mem::Mem;

pub fn parse_input(raw: &str) -> Vec<i128> {
    raw.trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

mod mem {
    use std::{collections::HashMap, sync::Arc};

    #[derive(Clone)]
    pub struct Mem {
        data: Arc<MemData>,
    }

    impl Mem {
        pub fn new(initial_data: Vec<i128>) -> Self {
            Self {
                data: Arc::new(MemData::new(initial_data)),
            }
        }

        pub fn get(&self, index: usize) -> &i128 {
            self.data.get(index)
        }

        pub fn get_mut(&mut self, index: usize) -> &mut i128 {
            self.data.make_mut().get_mut(index)
        }

        pub fn range<B: std::iter::Iterator<Item = usize>>(&self, range: B) -> Box<[i128]> {
            let mut v = Vec::new();
            for i in range {
                v.push(self[i]);
            }
            v.into_boxed_slice()
        }
    }

    impl std::ops::Index<usize> for Mem {
        type Output = i128;

        fn index(&self, index: usize) -> &Self::Output {
            self.get(index)
        }
    }

    impl std::ops::IndexMut<usize> for Mem {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.get_mut(index)
        }
    }

    impl std::fmt::Debug for Mem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data: &MemData = &self.data;
            f.debug_struct("Mem").field("data", data).finish()
        }
    }

    enum Parent {
        MemData(Arc<MemData>),
        Initial(Vec<i128>),
    }

    impl Parent {
        fn get(&self, index: usize) -> &i128 {
            match self {
                Parent::MemData(mem_data) => mem_data.get(index),
                Parent::Initial(initial_data) => initial_data.get(index).unwrap_or(&0),
            }
        }
    }

    struct MemData {
        parent: Parent,
        data: HashMap<usize, i128>,
    }

    impl MemData {
        fn new(initial_data: Vec<i128>) -> Self {
            Self {
                parent: Parent::Initial(initial_data),
                data: HashMap::new(),
            }
        }

        fn get(&self, index: usize) -> &i128 {
            if let Some(v) = self.data.get(&index) {
                v
            } else {
                self.parent.get(index)
            }
        }

        fn len(&self) -> usize {
            let my_len = self.data.keys().max().map(|x| x + 1).unwrap_or(0);
            let parent_len = match &self.parent {
                Parent::MemData(mem_data) => mem_data.len(),
                Parent::Initial(initial_data) => initial_data.len(),
            };

            my_len.max(parent_len)
        }

        fn get_mut(&mut self, index: usize) -> &mut i128 {
            self.data
                .entry(index)
                .or_insert_with(|| *self.parent.get(index))
        }

        fn make_mut<'a>(self: &'a mut Arc<MemData>) -> &'a mut MemData {
            if Arc::get_mut(self).is_none() {
                let parent = self.clone();
                *self = Arc::new(MemData {
                    parent: Parent::MemData(parent),
                    data: HashMap::new(),
                });
            }

            Arc::get_mut(self).unwrap()
        }
    }

    impl std::fmt::Debug for MemData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let len = self.len();
            let mut lst = f.debug_list();
            let mut lst = &mut lst;
            for i in 0..len {
                lst = lst.entry(self.get(i));
            }
            lst.finish()
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub mem: Mem,
    pub pos: usize,
    pub relative_base: i128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Code {
    Success,
    Stop,
}

fn get_args<const C: usize>(
    opcode: i128,
    mem: &Mem,
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
    pub fn new(mem: Vec<i128>) -> Self {
        // add zeros on the end for working memory
        Self {
            mem: Mem::new(mem),
            pos: 0,
            relative_base: 0,
        }
    }

    pub fn execute_operation(
        &mut self,
        mut input: impl FnMut() -> i128,
        mut output: impl FnMut(i128),
    ) -> Code {
        match self.mem[self.pos] % 100 {
            1 => {
                let params = self.mem.range(self.pos + 1..=self.pos + 3).to_owned();
                let [arg_1, arg_2] = get_args(
                    self.mem[self.pos],
                    &self.mem,
                    &params[..2],
                    self.relative_base,
                );
                let output_addr = get_addr(self.mem[self.pos], &params, self.relative_base, 2);
                self.mem[output_addr] = arg_1 + arg_2;
                self.pos += 4;
                Code::Success
            }
            2 => {
                let params = self.mem.range(self.pos + 1..=self.pos + 3).to_owned();
                let [arg_1, arg_2] = get_args(
                    self.mem[self.pos],
                    &self.mem,
                    &params[..2],
                    self.relative_base,
                );
                let output_addr = get_addr(self.mem[self.pos], &params, self.relative_base, 2);
                self.mem[output_addr] = arg_1 * arg_2;
                self.pos += 4;
                Code::Success
            }
            3 => {
                let param = self.mem[self.pos + 1];
                let output_addr = get_addr(self.mem[self.pos], &[param], self.relative_base, 0);
                self.mem[output_addr] = input();
                self.pos += 2;
                Code::Success
            }
            4 => {
                let param = self.mem[self.pos + 1];
                let [arg] = get_args(self.mem[self.pos], &self.mem, &[param], self.relative_base);
                output(arg);
                self.pos += 2;
                Code::Success
            }
            5 => {
                let params = self.mem.range(self.pos + 1..=self.pos + 2);
                let [arg_1, arg_2] =
                    get_args(self.mem[self.pos], &self.mem, &params, self.relative_base);
                if arg_1 != 0 {
                    self.pos = arg_2 as usize;
                } else {
                    self.pos += 3;
                }
                Code::Success
            }
            6 => {
                let params = self.mem.range(self.pos + 1..=self.pos + 2);
                let [arg_1, arg_2] =
                    get_args(self.mem[self.pos], &self.mem, &params, self.relative_base);
                if arg_1 == 0 {
                    self.pos = arg_2 as usize;
                } else {
                    self.pos += 3;
                }
                Code::Success
            }
            7 => {
                let params = self.mem.range(self.pos + 1..=self.pos + 3);
                let [arg_1, arg_2] = get_args(
                    self.mem[self.pos],
                    &self.mem,
                    &params[..2],
                    self.relative_base,
                );
                let output_addr = get_addr(self.mem[self.pos], &params, self.relative_base, 2);
                self.mem[output_addr] = i128::from(arg_1 < arg_2);
                self.pos += 4;
                Code::Success
            }
            8 => {
                let params = self.mem.range(self.pos + 1..=self.pos + 3);
                let [arg_1, arg_2] = get_args(
                    self.mem[self.pos],
                    &self.mem,
                    &params[..2],
                    self.relative_base,
                );
                let output_addr = get_addr(self.mem[self.pos], &params, self.relative_base, 2);
                self.mem[output_addr] = i128::from(arg_1 == arg_2);
                self.pos += 4;
                Code::Success
            }
            9 => {
                let param = self.mem[self.pos + 1];
                let [arg] = get_args(self.mem[self.pos], &self.mem, &[param], self.relative_base);
                self.relative_base += arg;
                self.pos += 2;
                Code::Success
            }
            99 => Code::Stop,
            op => panic!("Unrecognised opcode {op} at {}", self.pos),
        }
    }

    pub fn run_one_in_one_out(&mut self, input: i128) -> Option<i128> {
        let mut input = Some(input);
        let mut output = None;
        let mut last_result = Code::Success;

        loop {
            match last_result {
                Code::Success => {
                    if output.is_some() {
                        break output;
                    }
                    last_result =
                        self.execute_operation(|| input.take().unwrap(), |x| output = Some(x))
                }
                Code::Stop => {
                    break output;
                }
            }
        }
    }

    pub fn run_to_completion(
        &mut self,
        mut input: impl FnMut() -> i128,
        mut output: impl FnMut(i128),
    ) {
        while self.execute_operation(&mut input, &mut output) == Code::Success {}
    }
}
