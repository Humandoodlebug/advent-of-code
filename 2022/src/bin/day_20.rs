use itertools::Itertools;
use util::PerfTimer;

fn input() -> Vec<i128> {
    util::get_day_input(20)
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let input = input();
    {
        let _timer = PerfTimer::new("Part 1");

        let mut state: Vec<(i128, bool)> = input.iter().map(|&x| (x, false)).collect();
        let mut i = 0;
        while i < state.len() {
            if state[i].1 {
                i += 1;
                continue;
            }
            if state[i].0 % (state.len() as i128 - 1) == 0 {
                state[i].1 = true;
                i += 1;
                continue;
            }

            let v = state.remove(i).0;
            let new_i = {
                let p = (v + (i as i128)) % state.len() as i128;
                if p < 0 {
                    (p + state.len() as i128) as usize
                } else {
                    p as usize
                }
            };
            state.insert(new_i, (v, true));

            if new_i <= i {
                i += 1;
            }
        }
        let final_state: Vec<i128> = state.iter().map(|x| x.0).collect();
        let final_state: Vec<i128> = final_state
            .iter()
            .copied()
            .skip_while(|&x| x != 0)
            .chain(final_state.iter().copied().take_while(|&x| x != 0))
            .collect();

        let part_1 = final_state[1000 % final_state.len()]
            + final_state[2000 % final_state.len()]
            + final_state[3000 % final_state.len()];
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let initial_state: Vec<(usize, i128)> =
            input.iter().map(|&x| x * 811589153).enumerate().collect();
        let mut state = initial_state.clone();

        for _ in 0..10 {
            for v in &initial_state {
                let (i, _) = state.iter().find_position(|&x| v == x).unwrap();

                if state[i].1 % (state.len() as i128 - 1) == 0 {
                    continue;
                }

                let v = state.remove(i);
                let new_i = {
                    let p = (v.1 + (i as i128)) % state.len() as i128;
                    if p < 0 {
                        (p + state.len() as i128) as usize
                    } else {
                        p as usize
                    }
                };
                state.insert(new_i, v);
            }
        }

        let final_state: Vec<i128> = state.into_iter().map(|(_, v)| v).collect();
        let final_state: Vec<i128> = final_state
            .iter()
            .copied()
            .skip_while(|&x| x != 0)
            .chain(final_state.iter().copied().take_while(|&x| x != 0))
            .collect();

        let part_2 = final_state[1000 % final_state.len()]
            + final_state[2000 % final_state.len()]
            + final_state[3000 % final_state.len()];
        println!("Part 2: {part_2}");
    }
}
