use std::cmp::Ordering;

use util::PerfTimer;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Integer(i64),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
            (Value::List(left), Value::List(right)) => {
                for (l, r) in left.iter().zip(right.iter()) {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                left.len().cmp(&right.len())
            }
            (left @ Value::List(_), Value::Integer(right)) => {
                left.cmp(&Value::List(vec![Value::Integer(*right)]))
            }
            (Value::Integer(left), right @ Value::List(_)) => {
                Value::List(vec![Value::Integer(*left)]).cmp(right)
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn input() -> Vec<(Value, Value)> {
    fn parse_value(left: &mut &str) -> Value {
        if left.starts_with('[') {
            *left = &left[1..];
            let mut ret = Vec::new();
            while !left.starts_with(']') {
                ret.push(parse_value(left));
                if left.starts_with(',') {
                    *left = &left[1..];
                }
            }
            *left = &left[1..];
            Value::List(ret)
        } else {
            let mut end = 0;
            while &left[end..end + 1] != "]" && &left[end..end + 1] != "," {
                end += 1;
            }
            let ret = Value::Integer(left[..end].parse().unwrap());
            *left = &left[end..];
            ret
        }
    }

    let raw = util::get_day_input(13);
    let pairs = raw.trim().split("\n\n");
    let mut ret = Vec::new();
    for (mut left, mut right) in pairs.map(|p| p.split_once('\n').unwrap()) {
        let left = parse_value(&mut left);
        let right = parse_value(&mut right);
        ret.push((left, right));
    }
    ret
}

fn main() {
    let packets = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let mut part_1 = 0;
        for (index, (left, right)) in (1..).zip(packets.iter()) {
            if left < right {
                part_1 += index;
            }
        }
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let mut packets: Vec<Value> = packets
            .into_iter()
            .flat_map(|(left, right)| [left, right])
            .collect();
        let divider_1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
        let divider_2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);
        packets.push(divider_1.clone());
        packets.push(divider_2.clone());
        packets.sort();
        let mut part_2 = 1;
        for (index, packet) in (1..).zip(packets.into_iter()) {
            if packet == divider_1 || packet == divider_2 {
                part_2 *= index;
            }
        }
        println!("Part 2: {part_2}");
    }
}
