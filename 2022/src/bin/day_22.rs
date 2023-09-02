use std::collections::HashMap;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl std::ops::Add<usize> for Direction {
    type Output = Direction;

    fn add(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.turn_right();
        }
        self
    }
}

impl std::ops::Sub<usize> for Direction {
    type Output = Direction;

    fn sub(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.turn_left();
        }
        self
    }
}

impl Direction {
    fn turn_right(&mut self) {
        use Direction::*;
        *self = match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn turn_left(&mut self) {
        use Direction::*;
        *self = match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Face {
    Top,    // top of face is back face
    Right,  // top of face is top face
    Left,   // top of face is top face
    Front,  // top of face is top face
    Back,   // top of face is top face
    Bottom, // top of face is back face
}

impl Face {
    fn face_in(
        &self,
        direction: Direction,
    ) -> (
        Face,
        usize, /* up rotation offset for new face relative to old */
    ) {
        match self {
            Face::Top => match direction {
                Direction::Right => (Face::Right, 3),
                Direction::Down => (Face::Front, 0),
                Direction::Left => (Face::Left, 1),
                Direction::Up => (Face::Back, 2),
            },
            Face::Right => match direction {
                Direction::Right => (Face::Back, 0),
                Direction::Down => (Face::Bottom, 1),
                Direction::Left => (Face::Front, 0),
                Direction::Up => (Face::Top, 1),
            },
            Face::Left => match direction {
                Direction::Right => (Face::Front, 0),
                Direction::Down => (Face::Bottom, 3),
                Direction::Left => (Face::Back, 0),
                Direction::Up => (Face::Top, 3),
            },
            Face::Front => match direction {
                Direction::Right => (Face::Right, 0),
                Direction::Down => (Face::Bottom, 2),
                Direction::Left => (Face::Left, 0),
                Direction::Up => (Face::Top, 0),
            },
            Face::Back => match direction {
                Direction::Right => (Face::Left, 0),
                Direction::Down => (Face::Bottom, 0),
                Direction::Left => (Face::Right, 0),
                Direction::Up => (Face::Top, 2),
            },
            Face::Bottom => match direction {
                Direction::Right => (Face::Left, 1),
                Direction::Down => (Face::Front, 2),
                Direction::Left => (Face::Right, 3),
                Direction::Up => (Face::Back, 0),
            },
        }
    }
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
            .find(|(_, &t)| t == Tile::Open)
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
            Instruction::TurnRight => self.direction.turn_right(),
            Instruction::TurnLeft => self.direction.turn_left(),
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
}

struct CubeStateBuilder<'a> {
    grid: &'a Vec<Vec<Tile>>,
    side_len: usize,
}

impl<'a> CubeStateBuilder<'a> {
    fn new(grid: &'a Vec<Vec<Tile>>) -> Self {
        let max_grid_dim = grid.len().max(grid[0].len());
        let side_len = max_grid_dim / 4;
        Self { grid, side_len }
    }

    fn rotate_right(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        (0..grid[0].len())
            .map(|i| {
                (0..grid.len())
                    .map(|j| grid[grid.len() - j - 1][i])
                    .collect()
            })
            .collect()
    }

    /// Extract a face from the grid. `up_is` indicates the direction on the grid that up is for this face.
    fn extract_face(&self, coords: (usize, usize), up_is: Direction) -> Vec<Vec<Tile>> {
        let mut face = (coords.0..coords.0 + self.side_len)
            .map(|i| {
                (coords.1..coords.1 + self.side_len)
                    .map(|j| self.grid[i][j])
                    .collect()
            })
            .collect();
        let rotations = match up_is {
            Direction::Up => 0,
            Direction::Right => 3,
            Direction::Down => 2,
            Direction::Left => 1,
        };

        for _ in 0..rotations {
            face = Self::rotate_right(&face);
        }
        face
    }

    fn find_first_face(&self) -> (usize, usize) {
        for (i, &tile) in self.grid[0].iter().enumerate().step_by(self.side_len) {
            if tile != Tile::Void {
                return (0, i);
            }
        }
        panic!();
    }

    fn find_next_face(
        &self,
        coords: (usize, usize),
        up_is: Direction, // direction on the grid that is up for the current face
        face: Face,
        search_face_direction: Direction, // direction to search in (relative to current face)
    ) -> Option<(
        Face,
        (usize, usize),
        Direction, /* up_is for new face */
    )> {
        let search_offset = match search_face_direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        let search_grid_direction = up_is + search_offset;

        // top left of face relative to grid
        let new_face_grid_coords = match search_grid_direction {
            Direction::Right => {
                let new_column = coords.1 + self.side_len;
                if new_column >= self.grid[0].len() {
                    return None;
                }
                (coords.0, new_column)
            }
            Direction::Down => {
                let new_row = coords.0 + self.side_len;
                if new_row >= self.grid.len() {
                    return None;
                }
                (new_row, coords.1)
            }
            Direction::Left => {
                if coords.1 < self.side_len {
                    return None;
                }
                (coords.0, coords.1 - self.side_len)
            }
            Direction::Up => {
                if coords.0 < self.side_len {
                    return None;
                }
                (coords.0 - self.side_len, coords.1)
            }
        };

        if self.grid[new_face_grid_coords.0][new_face_grid_coords.1] == Tile::Void {
            return None;
        }

        let (new_face, rotation_offset) = face.face_in(search_face_direction);
        let new_face_up_is = up_is + rotation_offset;

        Some((new_face, new_face_grid_coords, new_face_up_is))
    }

    fn build(&self) -> CubeState {
        let mut faces: HashMap<Face, Vec<Vec<Tile>>> = HashMap::new();
        let mut face_coords: HashMap<Face, (usize, usize)> = HashMap::new();
        let mut face_up_is: HashMap<Face, Direction> = HashMap::new();

        // (face, coords, up_is)
        let mut to_search: Vec<(Face, (usize, usize), Direction)> = Vec::new();

        // find first face (top face)
        let top_face_coords = self.find_first_face();
        let face = self.extract_face(top_face_coords, Direction::Up);
        faces.insert(Face::Top, face);
        face_coords.insert(Face::Top, top_face_coords);
        face_up_is.insert(Face::Top, Direction::Up);
        to_search.push((Face::Top, top_face_coords, Direction::Up));

        while let Some((face, coords, up_is)) = to_search.pop() {
            for direction in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                if let Some((new_face, new_coords, new_up_is)) =
                    self.find_next_face(coords, up_is, face, direction)
                {
                    if faces.contains_key(&new_face) {
                        continue;
                    }
                    let new_face_tiles = self.extract_face(new_coords, new_up_is);
                    faces.insert(new_face, new_face_tiles);
                    face_coords.insert(new_face, new_coords);
                    face_up_is.insert(new_face, new_up_is);
                    to_search.push((new_face, new_coords, new_up_is));
                }
            }
        }

        CubeState {
            faces,
            row: 0,
            column: 0,
            direction: Direction::Right,
            face: Face::Top,
            side_len: self.side_len,
            face_coords,
            face_up_is,
        }
    }
}

#[derive(Clone, Debug)]
struct CubeState {
    faces: HashMap<Face, Vec<Vec<Tile>>>,
    row: usize,
    column: usize,
    direction: Direction,
    face: Face,
    side_len: usize,
    face_coords: HashMap<Face, (usize, usize)>,
    /// The grid direction that each face is facing (up relative to the face is ____ relative to the grid).
    face_up_is: HashMap<Face, Direction>,
}

impl CubeState {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(x) => self.mv(x),
            Instruction::TurnRight => self.direction.turn_right(),
            Instruction::TurnLeft => self.direction.turn_left(),
        }
    }

    fn mv(&mut self, spaces: usize) {
        for _ in 0..spaces {
            let (face, row, column, direction) = match self.direction {
                Direction::Right => {
                    let column = self.column + 1;
                    if column < self.side_len {
                        (self.face, self.row, column, self.direction)
                    } else {
                        match self.face {
                            Face::Top => (
                                Face::Right,
                                0,
                                self.side_len - self.row - 1,
                                Direction::Down,
                            ),
                            Face::Right => (Face::Back, self.row, 0, Direction::Right),
                            Face::Left => (Face::Front, self.row, 0, Direction::Right),
                            Face::Front => (Face::Right, self.row, 0, Direction::Right),
                            Face::Back => (Face::Left, self.row, 0, Direction::Right),
                            Face::Bottom => {
                                (Face::Left, self.side_len - 1, self.row, Direction::Up)
                            }
                        }
                    }
                }
                Direction::Down => {
                    let row = self.row + 1;
                    if row < self.side_len {
                        (self.face, row, self.column, self.direction)
                    } else {
                        match self.face {
                            Face::Top => (Face::Front, 0, self.column, Direction::Down),
                            Face::Right => (
                                Face::Bottom,
                                self.side_len - self.column - 1,
                                0,
                                Direction::Right,
                            ),
                            Face::Left => (
                                Face::Bottom,
                                self.column,
                                self.side_len - 1,
                                Direction::Left,
                            ),
                            Face::Front => (
                                Face::Bottom,
                                self.side_len - 1,
                                self.side_len - self.column - 1,
                                Direction::Up,
                            ),
                            Face::Back => (Face::Bottom, 0, self.column, Direction::Down),
                            Face::Bottom => (
                                Face::Front,
                                self.side_len - 1,
                                self.side_len - self.column - 1,
                                Direction::Up,
                            ),
                        }
                    }
                }
                Direction::Left => {
                    if self.column != 0 {
                        (self.face, self.row, self.column - 1, self.direction)
                    } else {
                        match self.face {
                            Face::Top => (Face::Left, 0, self.row, Direction::Down),
                            Face::Right => {
                                (Face::Front, self.row, self.side_len - 1, Direction::Left)
                            }
                            Face::Left => {
                                (Face::Back, self.row, self.side_len - 1, Direction::Left)
                            }
                            Face::Front => {
                                (Face::Left, self.row, self.side_len - 1, Direction::Left)
                            }
                            Face::Back => {
                                (Face::Right, self.row, self.side_len - 1, Direction::Left)
                            }
                            Face::Bottom => (
                                Face::Right,
                                self.side_len - 1,
                                self.side_len - self.row - 1,
                                Direction::Up,
                            ),
                        }
                    }
                }
                Direction::Up => {
                    if self.row != 0 {
                        (self.face, self.row - 1, self.column, self.direction)
                    } else {
                        match self.face {
                            Face::Top => (
                                Face::Back,
                                0,
                                self.side_len - self.column - 1,
                                Direction::Down,
                            ),
                            Face::Right => (
                                Face::Top,
                                self.side_len - self.column - 1,
                                self.side_len - 1,
                                Direction::Left,
                            ),
                            Face::Left => (Face::Top, self.column, 0, Direction::Right),
                            Face::Front => {
                                (Face::Top, self.side_len - 1, self.column, Direction::Up)
                            }
                            Face::Back => (
                                Face::Top,
                                0,
                                self.side_len - self.column - 1,
                                Direction::Down,
                            ),
                            Face::Bottom => {
                                (Face::Back, self.side_len - 1, self.column, Direction::Up)
                            }
                        }
                    }
                }
            };
            let next_face = &self.faces[&face];
            if next_face[row][column] == Tile::Wall {
                break;
            }
            assert_eq!(next_face[row][column], Tile::Open);
            self.face = face;
            self.row = row;
            self.column = column;
            self.direction = direction;
        }
    }

    fn score(&self) -> usize {
        let (face_row, face_column) = self.face_coords[&self.face];
        let mut direction = self.direction;
        let right_rotations = match self.face_up_is[&self.face] {
            Direction::Up => 0,
            Direction::Right => 3,
            Direction::Down => 2,
            Direction::Left => 1,
        };
        let mut row = self.row;
        let mut column = self.column;
        for _ in 0..right_rotations {
            ((row, column), direction) = self.rotate_right((row, column), direction);
        }
        let final_row = face_row + row + 1;
        let final_column = face_column + column + 1;
        let final_facing = match direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        1000 * final_row + 4 * final_column + final_facing
    }

    fn rotate_right(
        &self,
        coords: (usize, usize),
        direction: Direction,
    ) -> ((usize, usize), Direction) {
        let new_coords = (self.side_len - coords.1 - 1, coords.0);

        let new_direction = direction - 1;
        (new_coords, new_direction)
    }
}

fn main() {
    let (grid, instructions) = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut state = State::new(&grid);
        for &instruction in &instructions {
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

        let part_1 = 1000 * final_row + 4 * final_column + final_facing;

        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut state = CubeStateBuilder::new(&grid).build();
        for (_, &instruction) in instructions.iter().enumerate() {
            state.execute(instruction);
        }

        let part_2 = state.score();

        println!("Part 2: {part_2}");
    }
}
