use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {
        panic!("Couldn't read input file");
    };

    println!("Day 01: {}", day01(data));
}

fn day01(input: String) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(n, line)| {
            let first_digit_pos = line
                .find(|x| char::is_digit(x, 10))
                .expect(&format!("Couldn't find any digit at line {n}"));
            let last_digit_pos = line
                .rfind(|x| char::is_digit(x, 10))
                .expect(&format!("Couldn't find any digit at line {n}"));

            let first_digit = line
                .chars()
                .nth(first_digit_pos)
                .expect(&format!("Failed to get first char of line {n}"));
            let last_digit = line
                .chars()
                .nth(last_digit_pos)
                .expect(&format!("Failed to get last char of line {n}"));

            format!("{first_digit}{last_digit}")
                .parse::<usize>()
                .expect("Failed to parse back to digit at index {n}")
        })
        .sum()
}
