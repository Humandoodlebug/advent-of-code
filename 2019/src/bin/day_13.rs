use std::collections::HashMap;

// use std::time::Duration;
// use crossterm::{
//     cursor,
//     event::{KeyCode, KeyModifiers},
// };

use util::{
    intcode::{self, State},
    PerfTimer,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i128> for Tile {
    fn from(i: i128) -> Self {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            x => panic!("Unrecognised tile ID {x:?}"),
        }
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Block => '%',
            Tile::HorizontalPaddle => '-',
            Tile::Ball => 'O',
        }
    }
}

fn main() {
    let input = intcode::parse_input(&util::get_day_input(13));

    {
        let _timer = PerfTimer::new("Part 1");
        let mut mem = input.clone();
        mem.extend((0..1000000).map(|_| 0));
        let mut screen = HashMap::new();
        let mut output_buffer = Vec::with_capacity(3);
        let mut state = State::new(mem);
        state.run_to_completion(
            || panic!("No input available"),
            |x| {
                output_buffer.push(x);
                if output_buffer.len() == 3 {
                    let x = output_buffer[0];
                    let y = output_buffer[1];
                    let tile = Tile::from(output_buffer[2]);
                    screen.insert((x, y), tile);
                    output_buffer.clear();
                }
            },
        );
        let part_1 = screen.values().filter(|&&t| t == Tile::Block).count();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut mem = input;
        #[allow(clippy::needless_range_loop)]
        for i in 1520..1558 {
            // Pay no attention to this...
            mem[i] = 3;
        }
        mem[0] = 2;
        let mut score = 0;
        let mut output_buffer = Vec::with_capacity(3);
        let mut state = State::new(mem);

        state.run_to_completion(
            || 0i128,
            |x| {
                output_buffer.push(x);
                if output_buffer.len() == 3 {
                    if output_buffer[0] == -1 && output_buffer[1] == 0 {
                        score = output_buffer[2];
                    }
                    output_buffer.clear();
                }
            },
        );
        println!("Part 2: {score}");

        // Interactive version
        // crossterm::terminal::enable_raw_mode().unwrap();
        // let mut stdout = std::io::stdout();
        // intcode::run_to_completion(
        //     mem,
        //     || {
        //         if crossterm::event::poll(Duration::from_secs(1)).unwrap() {
        //             match crossterm::event::read().unwrap() {
        //                 crossterm::event::Event::Key(event) => match event.code {
        //                     KeyCode::Left | KeyCode::Char('a') => -1,
        //                     KeyCode::Right | KeyCode::Char('d') => 1,
        //                     KeyCode::Char('c')
        //                         if event.modifiers.contains(KeyModifiers::CONTROL) =>
        //                     {
        //                         std::process::exit(1);
        //                     }
        //                     _ => 0,
        //                 },
        //                 _ => 0,
        //             }
        //         } else {
        //             0
        //         }
        //     },
        //     |x| {
        //         output_buffer.push(x);
        //         if output_buffer.len() == 3 {
        //             let x = output_buffer[0];
        //             let y = output_buffer[1];
        //             if x == -1 && y == 0 {
        //                 score = output_buffer[2];
        //                 crossterm::execute!(
        //                     stdout,
        //                     cursor::MoveTo(0, 0),
        //                     crossterm::style::Print(format!("Score: {score}"))
        //                 )
        //                 .unwrap();
        //                 output_buffer.clear();
        //             } else {
        //                 let tile = Tile::from(output_buffer[2]);
        //                 crossterm::execute!(
        //                     stdout,
        //                     cursor::MoveTo(x as u16, y as u16 + 1),
        //                     crossterm::style::Print::<char>(tile.into())
        //                 )
        //                 .unwrap();
        //                 output_buffer.clear();
        //             }
        //         }
        //     },
        // );
    }
}
