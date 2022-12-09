use std::collections::HashMap;

use util::{get_day_input, PerfTimer};

#[derive(Debug, PartialEq, Eq)]
enum Entry {
    Dir(String),
    File(String, u64),
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Cd(String),
    Ls(Vec<Entry>),
}

fn input() -> Vec<Command> {
    let raw = get_day_input(7);
    let mut commands = Vec::new();
    let mut current_dir_name = None;
    let mut current_dir_contents = None;
    for line in raw.lines() {
        let line: Vec<&str> = line.split(' ').collect();
        match line[0] {
            "$" => {
                if let Some(dir_name) = current_dir_name.take() {
                    commands.push(Command::Cd(dir_name));
                }
                if let Some(dir_contents) = current_dir_contents.take() {
                    commands.push(Command::Ls(dir_contents))
                }
                match line[1] {
                    "cd" => current_dir_name = Some(String::from(line[2])),
                    "ls" => current_dir_contents = Some(Vec::new()),
                    o => panic!("Unrecognised command {o:?}"),
                }
            }
            "dir" => current_dir_contents
                .as_mut()
                .unwrap()
                .push(Entry::Dir(String::from(line[1]))),
            o => {
                let size: u64 = o.parse().unwrap();
                let name = String::from(line[1]);
                current_dir_contents
                    .as_mut()
                    .unwrap()
                    .push(Entry::File(name, size));
            }
        }
    }
    if let Some(dir_name) = current_dir_name.take() {
        commands.push(Command::Cd(dir_name));
    }
    if let Some(dir_contents) = current_dir_contents.take() {
        commands.push(Command::Ls(dir_contents))
    }
    commands
}

enum FsItem {
    Dir(HashMap<String, FsItem>),
    File(u64),
}

impl FsItem {
    fn unwrap_dir(&self) -> &HashMap<String, FsItem> {
        match self {
            Self::Dir(m) => m,
            Self::File(_) => panic!(),
        }
    }

    fn unwrap_dir_mut(&mut self) -> &mut HashMap<String, FsItem> {
        match self {
            Self::Dir(m) => m,
            Self::File(_) => panic!(),
        }
    }
}

fn build_directories(mut commands: &[Command]) -> FsItem {
    let mut root_contents = HashMap::new();
    build_subdirectory(&mut commands, &mut root_contents, true);
    return FsItem::Dir(root_contents);

    fn build_subdirectory(
        commands: &mut &[Command],
        contents: &mut HashMap<String, FsItem>,
        is_root: bool,
    ) {
        while !commands.is_empty() {
            let command = &commands[0];
            match command {
                Command::Cd(loc) => match loc.as_str() {
                    "/" => {
                        if is_root {
                            *commands = &commands[1..];
                        } else {
                            return;
                        }
                    }
                    ".." => {
                        if is_root {
                            panic!();
                        } else {
                            *commands = &commands[1..];
                            return;
                        }
                    }
                    o => {
                        *commands = &commands[1..];
                        let o_contents = contents
                            .entry(String::from(o))
                            .or_insert_with(|| FsItem::Dir(HashMap::new()))
                            .unwrap_dir_mut();
                        build_subdirectory(commands, o_contents, false);
                    }
                },
                Command::Ls(entries) => {
                    *commands = &commands[1..];
                    for entry in entries {
                        match entry {
                            Entry::Dir(name) => {
                                contents
                                    .entry(String::from(name))
                                    .or_insert_with(|| FsItem::Dir(HashMap::new()));
                            }
                            Entry::File(name, size) => {
                                contents
                                    .entry(String::from(name))
                                    .or_insert(FsItem::File(*size));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn dir_sizes(directories: &HashMap<String, FsItem>) -> (u64, Vec<u64>) {
    let mut my_size = 0;
    let mut sub_sizes = Vec::new();
    for item in directories.values() {
        match item {
            FsItem::Dir(contents) => {
                let (o_size, mut o_sub_sizes) = dir_sizes(contents);
                my_size += o_size;
                sub_sizes.append(&mut o_sub_sizes);
            }
            FsItem::File(f_size) => {
                my_size += f_size;
            }
        }
    }
    sub_sizes.push(my_size);
    (my_size, sub_sizes)
}

fn main() {
    let commands = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let directories = build_directories(&commands);
        let part_1: u64 = dir_sizes(directories.unwrap_dir())
            .1
            .iter()
            .filter(|&&x| x <= 100_000)
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let directories = build_directories(&commands);
        let (used_space, mut dir_sizes) = dir_sizes(directories.unwrap_dir());
        let remaining_space = 70_000_000 - used_space;
        let space_to_clear = 30_000_000 - remaining_space;
        dir_sizes.sort();
        let part_2 = dir_sizes
            .into_iter()
            .find(|&x| x >= space_to_clear)
            .unwrap();
        println!("Part 2: {part_2}");
    }
}
