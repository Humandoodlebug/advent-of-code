use util::PerfTimer;

extern crate util;

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Vec<bool>>,
    default: bool,
}

impl Image {
    fn get(&self, i: i32, j: i32) -> bool {
        if i >= 0 && j >= 0 && i < self.pixels.len() as i32 && j < self.pixels.len() as i32 {
            self.pixels[i as usize][j as usize]
        } else {
            self.default
        }
    }
}

fn input() -> (Vec<bool>, Vec<Vec<bool>>) {
    let raw = util::get_day_input(20);
    let mut lines = raw.lines();
    let raw_alg = lines.next().unwrap();
    let alg: Vec<bool> = raw_alg.chars().map(parse_symbol).collect();

    fn parse_symbol(c: char) -> bool {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!(),
        }
    }

    assert!(lines.next().unwrap().is_empty());

    let image: Vec<Vec<bool>> = lines
        .map(|l| l.chars().map(parse_symbol).collect())
        .collect();
    (alg, image)
}

fn enhance(alg: &[bool], image: &Image) -> Image {
    let mut new_image = Image {
        default: false,
        pixels: vec![vec![false; image.pixels[0].len() + 4]; image.pixels.len() + 4],
    };
    for x in 0..image.pixels.len() + 4 {
        for y in 0..image.pixels[0].len() + 4 {
            let x_old = x as i32 - 2;
            let y_old = y as i32 - 2;

            let mut num = 0;
            for i in x_old - 1..=x_old + 1 {
                for j in y_old - 1..=y_old + 1 {
                    num *= 2;
                    if image.get(i, j) {
                        num += 1;
                    }
                }
            }
            new_image.pixels[x][y] = alg[num];
            if image.default {
                new_image.default = alg[511];
            } else {
                new_image.default = alg[0];
            }
        }
    }
    new_image
}

fn main() {
    let (alg, pixels) = input();
    let image = Image {
        default: false,
        pixels,
    };

    {
        let _timer = PerfTimer::new("Part 1");
        for l in enhance(&alg, &image).pixels {
            println!(
                "{}",
                l.iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            );
        }
        let result = enhance(&alg, &enhance(&alg, &image));

        let part1 = result.pixels.iter().flatten().filter(|&&x| x).count();
        println!("Part 1: {}", part1);
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut image = image;
        for _ in 0..50 {
            image = enhance(&alg, &image);
        }
        let part2 = image.pixels.iter().flatten().filter(|&&x| x).count();
        println!("Part 2: {}", part2);
    }
}
