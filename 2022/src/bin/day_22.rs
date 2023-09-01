use util::PerfTimer;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Void,
    Open,
    Wall,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Move(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn input() -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let raw = util::get_day_input(22);
    let raw_rows: Vec<&str> = raw.lines().take_while(|l| !l.trim().is_empty()).collect();
    let raw_instructions = raw.lines().last().unwrap();

    let row_count = raw_rows.len();
    let column_count = raw_rows.iter().map(|r| r.len()).max().unwrap();

    let mut grid: Vec<Vec<Tile>> = (0..row_count)
        .map(|_| (0..column_count).map(|_| Tile::Void).collect())
        .collect();
    for (i, raw_row) in raw_rows.iter().copied().enumerate() {
        for (j, c) in raw_row.chars().enumerate() {
            grid[i][j] = match c {
                ' ' => Tile::Void,
                '.' => Tile::Open,
                '#' => Tile::Wall,
                c => panic!("unrecognised tile {c:?}"),
            }
        }
    }

    let mut instructions: Vec<Instruction> = Vec::new();

    let mut next_move = String::new();
    for c in raw_instructions.chars() {
        if c.is_numeric() {
            next_move.push(c);
        } else {
            if !next_move.is_empty() {
                let v: usize = next_move.parse().unwrap();
                instructions.push(Instruction::Move(v));
                next_move.clear();
            }
            match c {
                'R' => instructions.push(Instruction::TurnRight),
                'L' => instructions.push(Instruction::TurnLeft),
                c => panic!("unrecognised instruction {c:?}"),
            }
        }
    }
    if !next_move.is_empty() {
        let v: usize = next_move.parse().unwrap();
        instructions.push(Instruction::Move(v));
    }

    (grid, instructions)
}

struct State<'a> {
    grid: &'a Vec<Vec<Tile>>,
    row: usize,
    column: usize,
    direction: Direction,
}

impl<'a> State<'a> {
    pub fn new(grid: &'a Vec<Vec<Tile>>) -> Self {
        let column = grid[0]
            .iter()
            .enumerate()
            .skip_while(|(_, &t)| t != Tile::Open)
            .next()
            .unwrap()
            .0;
        State {
            grid,
            row: 0,
            column,
            direction: Direction::Right,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(x) => self.mv(x),
            Instruction::TurnRight => self.turn_right(),
            Instruction::TurnLeft => self.turn_left(),
        }
    }

    fn mv(&mut self, spaces: usize) {
        for _ in 0..spaces {
            let (mut row, mut column) = (self.row, self.column);
            loop {
                (row, column) = match self.direction {
                    Direction::Right => (row, (column + 1) % self.grid[row].len()),
                    Direction::Down => ((row + 1) % self.grid.len(), column),
                    Direction::Left => (
                        row,
                        if column == 0 {
                            self.grid[row].len() - 1
                        } else {
                            column - 1
                        },
                    ),
                    Direction::Up => (
                        if row == 0 {
                            self.grid.len() - 1
                        } else {
                            row - 1
                        },
                        column,
                    ),
                };
                if self.grid[row][column] != Tile::Void {
                    break;
                }
            }
            if self.grid[row][column] == Tile::Wall {
                break;
            }
            assert_eq!(self.grid[row][column], Tile::Open);
            self.row = row;
            self.column = column;
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

fn main() {
    let (grid, instructions) = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut state = State::new(&grid);
        for instruction in instructions {
            state.execute(instruction);
        }

        let final_row = state.row + 1;
        let final_column = state.column + 1;
        let final_facing = match state.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        dbg!(final_row, final_column, final_facing);

        let part_1 = 1000 * final_row + 4 * final_column + final_facing;

        println!("Part 1: {part_1}");
    }
}
