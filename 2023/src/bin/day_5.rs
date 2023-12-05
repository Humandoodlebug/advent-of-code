use util::PerfTimer;

#[allow(dead_code)]
struct Map {
    source_name: String,
    dest_name: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, number: u64) -> u64 {
        for range in &self.ranges {
            if number >= range.source_start && number < range.source_start + range.length {
                return range.dest_start + (number - range.source_start);
            }
        }
        number
    }
    fn map_range(&self, start: u64, length: u64) -> Vec<(u64, u64)> {
        let mut input_ranges = vec![(start, length)];
        let mut output_ranges = Vec::new();
        for range in &self.ranges {
            let mut new_input_ranges = Vec::new();
            for (start, length) in input_ranges {
                let overlap_start = start.max(range.source_start);
                let overlap_end = (start + length).min(range.source_start + range.length);
                if overlap_start < overlap_end {
                    let new_start = range.dest_start + (overlap_start - range.source_start);
                    let new_length = overlap_end - overlap_start;
                    output_ranges.push((new_start, new_length));
                    if overlap_start > start {
                        new_input_ranges.push((start, overlap_start - start));
                    }
                    if overlap_end < start + length {
                        new_input_ranges.push((overlap_end, start + length - overlap_end));
                    }
                } else {
                    new_input_ranges.push((start, length));
                }
            }
            input_ranges = new_input_ranges;
        }
        output_ranges.append(&mut input_ranges);
        output_ranges
    }
}

struct MapRange {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

fn input() -> (Vec<u64>, Vec<Map>) {
    let raw = util::get_day_input(5);
    let mut lines = raw.lines();
    let seed_line = lines.next().unwrap();
    let seeds = seed_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .unwrap();
    assert!(lines.next().unwrap().is_empty());
    let mut maps = Vec::new();
    while let Some(title_line) = lines.next() {
        let split: Vec<&str> = title_line
            .strip_suffix(" map:")
            .unwrap()
            .split('-')
            .collect();
        assert_eq!(split.len(), 3);
        assert_eq!(split[1], "to");
        let source_name = String::from(split[0]);
        let dest_name = String::from(split[2]);
        let mut ranges = Vec::new();
        for range_line in &mut lines {
            if range_line.is_empty() {
                break;
            }
            let split: Vec<&str> = range_line.split_whitespace().collect();
            assert_eq!(split.len(), 3);
            let dest_start = split[0].parse().unwrap();
            let source_start = split[1].parse().unwrap();
            let length = split[2].parse().unwrap();
            ranges.push(MapRange {
                source_start,
                dest_start,
                length,
            })
        }
        maps.push(Map {
            source_name,
            dest_name,
            ranges,
        })
    }
    (seeds, maps)
}

fn main() {
    let (seeds, maps) = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let locations = maps.iter().fold(seeds.clone(), |mut source_nums, map| {
            for source_num in &mut source_nums {
                *source_num = map.map(*source_num);
            }
            source_nums
        });
        let part_1 = *locations.iter().min().unwrap();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let seed_ranges: Vec<(u64, u64)> = seeds
            .chunks_exact(2)
            .map(|chunks| (chunks[0], chunks[1]))
            .collect();

        let location_ranges = maps.iter().fold(seed_ranges, |source_ranges, map| {
            source_ranges
                .into_iter()
                .flat_map(|(start, length)| map.map_range(start, length))
                .collect()
        });
        let part_2 = location_ranges
            .into_iter()
            .map(|(start, _)| start)
            .min()
            .unwrap();
        println!("Part 2: {part_2}");
    }
}
