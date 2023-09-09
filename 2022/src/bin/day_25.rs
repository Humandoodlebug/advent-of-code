use util::PerfTimer;

fn input() -> Vec<String> {
    let snafu_nums = util::get_day_input(25);
    snafu_nums.lines().map(String::from).collect()
}

fn from_snafu(snafu: &str) -> i64 {
    let mut v: i64 = 0;
    for c in snafu.chars() {
        v *= 5;
        v += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        }
    }
    v
}

fn to_snafu(mut decimal: i64) -> String {
    let mut snafu = String::new();
    while decimal != 0 {
        let (snafu_digit, snafu_digit_decimal_value) = match decimal % 5 {
            0 => ('0', 0),
            1 => ('1', 1),
            2 => ('2', 2),
            3 => ('=', -2),
            4 => ('-', -1),
            _ => panic!(),
        };
        snafu.push(snafu_digit);
        decimal -= snafu_digit_decimal_value;
        assert_eq!(decimal % 5, 0);
        decimal /= 5;
    }
    snafu.chars().rev().collect()
}

fn main() {
    let snafu_nums = input();
    let _timer = PerfTimer::new("Part 1");
    let sum: i64 = snafu_nums.iter().map(|x| from_snafu(x)).sum();
    let part_1 = to_snafu(sum);
    println!("Part 1: {part_1}");
}

#[cfg(test)]
mod tests {
    use crate::{from_snafu, to_snafu};

    #[test]
    fn test_from_snafu() {
        assert_eq!(from_snafu("1"), 1);
        assert_eq!(from_snafu("2"), 2);
        assert_eq!(from_snafu("1="), 3);
        assert_eq!(from_snafu("1-"), 4);
        assert_eq!(from_snafu("10"), 5);
        assert_eq!(from_snafu("1=-0-2"), 1747);
        assert_eq!(from_snafu("12111"), 906);
        assert_eq!(from_snafu("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(1747), "1=-0-2");
        assert_eq!(to_snafu(906), "12111");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }
}
