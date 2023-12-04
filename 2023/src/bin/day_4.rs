use util::PerfTimer;

struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn input() -> Vec<Card> {
    let input = util::get_day_input(4);
    let cards = input
        .lines()
        .map(|line| {
            let (id, numbers) = line.split_once(':').unwrap();
            let id: usize = id[5..].trim().parse().unwrap();
            let (winning_numbers, numbers) = numbers.split_once(" | ").unwrap();
            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u32>, _>>()
                .unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<u32>, _>>()
                .unwrap();

            Card {
                id,
                winning_numbers,
                numbers,
            }
        })
        .collect();
    cards
}

fn main() {
    let cards = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1: u32 = cards
            .iter()
            .map(|card| {
                let mut score = 0;
                for number in &card.numbers {
                    if card.winning_numbers.contains(number) {
                        if score == 0 {
                            score = 1;
                        } else {
                            score *= 2;
                        }
                    }
                }
                score
            })
            .sum();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut card_counts = vec![1; cards.len()];
        for card in &cards {
            let matches = card
                .numbers
                .iter()
                .filter(|number| card.winning_numbers.contains(number))
                .count();
            let card_count = card_counts[card.id - 1];
            for count in card_counts[card.id..card.id + matches].iter_mut() {
                *count += card_count;
            }
        }
        let part_2: u32 = card_counts.iter().sum();
        println!("Part 2: {part_2}");
    }
}
