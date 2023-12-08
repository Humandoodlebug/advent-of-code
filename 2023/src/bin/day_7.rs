use itertools::Itertools;
use util::PerfTimer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Wild,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandClass {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn input() -> Vec<(Vec<Card>, u64)> {
    util::get_day_input(7)
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand: Vec<Card> = hand
                .chars()
                .map(|c| match c {
                    '2' => Card::N2,
                    '3' => Card::N3,
                    '4' => Card::N4,
                    '5' => Card::N5,
                    '6' => Card::N6,
                    '7' => Card::N7,
                    '8' => Card::N8,
                    '9' => Card::N9,
                    'T' => Card::N10,
                    'J' => Card::Jack,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!(),
                })
                .collect();
            let bid: u64 = bid.parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn classify_hand(hand: &[Card]) -> HandClass {
    assert_eq!(hand.len(), 5);

    let wildcards = hand.iter().filter(|&&c| c == Card::Wild).count();
    if wildcards == 5 {
        return HandClass::FiveOfAKind;
    }

    let hand = hand
        .iter()
        .copied()
        .filter(|&c| c != Card::Wild)
        .collect_vec();

    let mut counts = hand
        .iter()
        .copied()
        .counts()
        .values()
        .copied()
        .sorted()
        .rev()
        .collect_vec();

    counts[0] += wildcards;

    match counts.as_slice() {
        [5] => HandClass::FiveOfAKind,
        [4, ..] => HandClass::FourOfAKind,
        [3, 2] => HandClass::FullHouse,
        [3, ..] => HandClass::ThreeOfAKind,
        [2, 2, ..] => HandClass::TwoPair,
        [2, ..] => HandClass::OnePair,
        [1, ..] => HandClass::HighCard,
        _ => unreachable!(),
    }
}

fn main() {
    let games = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = games
            .iter()
            .map(|(hand, bid)| {
                let class = classify_hand(hand);
                let sort_pair = (class, hand);
                (sort_pair, *bid)
            })
            .sorted()
            .zip(1..)
            .map(|((_, bid), i)| bid * i)
            .sum::<u64>();

        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let part_2 = games
            .into_iter()
            .map(|(mut hand, bid)| {
                for card in &mut hand {
                    if *card == Card::Jack {
                        *card = Card::Wild;
                    }
                }
                let class = classify_hand(&hand);
                let sort_pair = (class, hand);
                (sort_pair, bid)
            })
            .sorted()
            .zip(1..)
            .map(|((_, bid), i)| bid * i)
            .sum::<u64>();

        println!("Part 2: {part_2}");
    }
}
