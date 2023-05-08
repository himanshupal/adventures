use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else { return };

    {
        let mut cycle = 1;
        let mut acc = 1;

        let mut next_check_at = 20;
        let mut strength = 0;

        data.lines().for_each(|line| {
            let mut check = |cycle: i32| {
                if cycle == next_check_at {
                    strength += cycle * acc;
                    next_check_at += 40;
                }
            };

            if !line.eq("noop") {
                for _ in 0..=1 {
                    check(cycle);
                    cycle += 1;
                }
                let value = line.split(" ").nth(1).expect("Couldn't split the line");
                acc += value.parse::<i32>().expect("Couldn't parse to i32");
            } else {
                check(cycle);
                cycle += 1;
            }
        });

        println!("part_01: {strength}");
    }

    {
        let mut screen = vec![0u64; 6];

        let mut sprite_at = 1;
        let mut cycle = 1;

        println!("part_02:");

        data.lines().for_each(|line| {
            let mut check = |cycle: i32| {
                let current_cycle = if cycle % 40 == 0 { 40 } else { cycle % 40 };
                if current_cycle >= sprite_at && current_cycle <= (sprite_at + 2) {
                    screen[(cycle - 1) as usize / 40] |= 1 << (40 - current_cycle);
                };
            };

            if !line.eq("noop") {
                for _ in 0..=1 {
                    check(cycle);
                    cycle += 1;
                }
                let value = line.split(" ").nth(1).expect("Couldn't split the line");
                sprite_at += value.parse::<i32>().expect("Couldn't parse to i32");
            } else {
                check(cycle);
                cycle += 1;
            }
        });

        screen.iter().for_each(print_bitmap)
    }
}

fn print_bitmap(map: &u64) {
    let row_string = &map
        .to_le_bytes()
        .iter()
        .map(|bytes| {
            format!("{:08b}", bytes)
                .chars()
                .map(|ch| if ch == '1' { 'X' } else { ' ' })
                .collect()
        })
        .reduce(|p, c| format!("{c}{p}"))
        .expect("Couldn't build string from bitmap")[(64 - 40)..];

    println!("{row_string}");
}
