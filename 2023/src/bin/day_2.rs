use util::PerfTimer;

#[derive(Default)]
struct Counts {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    handfuls: Vec<Counts>,
}

fn input() -> Vec<Game> {
    let raw = util::get_day_input(2);
    raw.lines()
        .map(|line| {
            assert!(line.starts_with("Game "));
            let id = line[5..]
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            let rest = line.split_once(": ").unwrap().1;
            let handfuls = rest
                .split("; ")
                .map(|s| {
                    let mut counts = Counts::default();
                    for part in s.split(", ") {
                        let (count, colour) = part.split_once(' ').unwrap();
                        let count = count.parse::<u32>().unwrap();
                        match colour {
                            "red" => counts.red += count,
                            "green" => counts.green += count,
                            "blue" => counts.blue += count,
                            _ => panic!(),
                        };
                    }
                    counts
                })
                .collect();
            Game { id, handfuls }
        })
        .collect()
}

fn main() {
    let games = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: u32 = games
            .iter()
            .filter(|game| {
                game.handfuls
                    .iter()
                    .all(|counts| counts.red <= 12 && counts.green <= 13 && counts.blue <= 14)
            })
            .map(|game| game.id)
            .sum();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let part_2: u64 = games
            .iter()
            .map(|game| {
                let max = game
                    .handfuls
                    .iter()
                    .fold(Counts::default(), |mut max, counts| {
                        max.red = max.red.max(counts.red);
                        max.green = max.green.max(counts.green);
                        max.blue = max.blue.max(counts.blue);
                        max
                    });
                u64::from(max.red * max.green * max.blue)
            })
            .sum();
        println!("Part 2: {:?}", part_2);
    }
}
