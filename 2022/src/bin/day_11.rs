use util::PerfTimer;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(i128),
    Multiply(i128),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    test_divisor: i128,
    true_monkey: usize,
    false_monkey: usize,
}

fn input() -> Vec<Monkey> {
    util::get_day_input(11)
        .trim()
        .split("\n\n")
        .map(|chunk| {
            let lines: Vec<&str> = chunk.lines().collect();
            let items: Vec<i128> = lines[1][18..]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            let operation = match &lines[2][23..24] {
                "*" if lines[2].len() >= 28 && &lines[2][25..28] == "old" => Operation::Square,
                "+" => Operation::Add(lines[2][25..].parse().unwrap()),
                "*" => Operation::Multiply(lines[2][25..].parse().unwrap()),
                o => panic!("Unrecognised operator {o:?}"),
            };
            let test_divisor: i128 = lines[3][21..].parse().unwrap();
            let true_monkey: usize = lines[4][29..].parse().unwrap();
            let false_monkey: usize = lines[5][30..].parse().unwrap();
            Monkey {
                items,
                operation,
                test_divisor,
                true_monkey,
                false_monkey,
            }
        })
        .collect()
}

fn step(monkeys: &mut [Monkey], inspections: &mut [usize], big_divisor: i128, divide_by_3: bool) {
    for i in 0..monkeys.len() {
        let items: Vec<i128> = monkeys[i].items.drain(..).collect();
        for item in items {
            let new_item = (match monkeys[i].operation {
                Operation::Add(x) => item + x,
                Operation::Multiply(x) => item * x,
                Operation::Square => item * item,
            } / if divide_by_3 { 3 } else { 1 })
                % big_divisor;
            let new_monkey = if new_item % monkeys[i].test_divisor == 0 {
                monkeys[i].true_monkey
            } else {
                monkeys[i].false_monkey
            };
            monkeys[new_monkey].items.push(new_item);
            inspections[i] += 1;
        }
    }
}

fn main() {
    let monkeys = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut monkeys = monkeys.clone();
        let mut inspections = vec![0usize; monkeys.len()];
        for _ in 0..20 {
            step(&mut monkeys, &mut inspections, 1, true);
        }
        inspections.sort();
        let part_1 = inspections[inspections.len() - 1] * inspections[inspections.len() - 2];
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut monkeys = monkeys;
        let mut inspections = vec![0usize; monkeys.len()];
        let big_divisor = monkeys.iter().map(|m| m.test_divisor).product();
        for _ in 0..10_000 {
            step(&mut monkeys, &mut inspections, big_divisor, false);
        }
        println!("{inspections:?}");
        inspections.sort();
        let part_2 = inspections[inspections.len() - 1] * inspections[inspections.len() - 2];
        println!("Part 2: {part_2}");
    }
}
