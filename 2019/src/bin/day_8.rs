use util::PerfTimer;

fn input() -> Vec<u8> {
    let raw = util::get_day_input(8);
    raw.trim()
        .chars()
        .map(|c| (c as u32 - '0' as u32) as u8)
        .collect()
}

fn main() {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let input = input();

    {
        let _timer = PerfTimer::new("Part 1");

        let part_1_layer = input
            .chunks(WIDTH * HEIGHT)
            .min_by_key(|l| l.iter().copied().filter(|&p| p == 0).count())
            .unwrap();
        let ones = part_1_layer.iter().filter(|&&p| p == 1).count();
        let twos = part_1_layer.iter().filter(|&&p| p == 2).count();
        let part_1 = ones * twos;

        println!("Part 1: {part_1}");
    }

    {
        let layers: Vec<&[u8]> = input.chunks(WIDTH * HEIGHT).collect();
        let mut image = vec![2; WIDTH * HEIGHT];
        for &layer in layers.iter().rev() {
            for (pi, &pl) in image.iter_mut().zip(layer.iter()) {
                match pl {
                    0 => *pi = 0,
                    1 => *pi = 1,
                    2 => {}
                    x => panic!("Unhandled pixel value {x:?}"),
                }
            }
        }
        {
            let _timer = PerfTimer::new("Part 2");
            println!("Part 2:");
            for row in image.chunks(WIDTH) {
                for &p in row {
                    match p {
                        0 => print!(" "),
                        1 => print!("#"),
                        x => panic!("Unexpected pixel value {x:?}"),
                    }
                }
                println!();
            }
        }
    }
}
