use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else { return };

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

    println!("part_01: {strength}")
}
