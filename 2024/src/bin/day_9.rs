use util::PerfTimer;

fn input() -> Vec<u32> {
    util::get_day_input(9)
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn main() {
    let input = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut disk = Vec::new();
        for (i, n) in input.iter().copied().enumerate() {
            let v = if i % 2 == 0 { Some(i / 2) } else { None };
            for _ in 0..n {
                disk.push(v);
            }
        }

        let mut i = 0;
        'outer: loop {
            while disk.last().unwrap().is_none() {
                disk.pop();
            }
            while disk[i].is_some() {
                i += 1;
                if i >= disk.len() - 1 {
                    break 'outer;
                }
            }
            let last = disk.pop().unwrap();
            disk[i] = last;
        }

        let part_1 = disk
            .into_iter()
            .enumerate()
            .map(|(i, v)| i * v.unwrap())
            .sum::<usize>();

        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let mut disk = Vec::new();
        for (i, n) in input.iter().copied().enumerate() {
            let v = if i % 2 == 0 { Some(i / 2) } else { None };
            for _ in 0..n {
                disk.push(v);
            }
        }

        let mut from_cursor = disk.len() - 1;
        'outer: for disk_id in (1..=(disk.len() - 1) / 2).rev() {
            while disk[from_cursor].is_none_or(|v| v > disk_id) {
                from_cursor -= 1;
            }
            if disk[from_cursor] != Some(disk_id) {
                continue;
            }
            let last_block = from_cursor;
            while disk[from_cursor] == Some(disk_id) {
                from_cursor -= 1;
            }
            let first_block = from_cursor + 1;

            let file_len = last_block - first_block + 1;

            // Search for space to move the file to
            let mut to_cursor = 0;
            loop {
                while disk[to_cursor].is_some() {
                    if to_cursor == first_block {
                        // No space found before existing file location!
                        continue 'outer;
                    }
                    to_cursor += 1;
                }
                if first_block - to_cursor < last_block - first_block + 1 {
                    // Not enough space to move the file!
                    continue 'outer;
                }

                let dest_start = to_cursor;
                while disk[to_cursor].is_none() {
                    to_cursor += 1;
                    if to_cursor - dest_start == file_len {
                        // Move the file
                        disk.copy_within(first_block..=last_block, dest_start);
                        disk[first_block..=last_block].fill(None);
                        continue 'outer;
                    }
                }
            }
        }

        let part_2 = disk
            .into_iter()
            .enumerate()
            .map(|(i, v)| i * v.unwrap_or(0))
            .sum::<usize>();

        println!("Part 2: {part_2}");
    }
}
