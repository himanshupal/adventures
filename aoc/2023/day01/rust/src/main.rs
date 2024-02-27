use std::{cmp::Ordering, fs};

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {
        panic!("Couldn't read input file");
    };

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(input: &String) -> usize {
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

fn part2(input: &String) -> usize {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            let mut values = (3..=5)
                .map(|size| {
                    chars
                        .windows(size)
                        .enumerate()
                        .filter_map(|(n, chunk)| {
                            let chunk_as_str = String::from_iter(chunk);

                            if numbers.contains(&chunk_as_str.as_str()) {
                                Some((n, chunk_as_str))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(usize, String)>>()
                })
                .fold(vec![], |mut p, mut c| {
                    p.append(&mut c);
                    p
                });

            if let Some(value) = line.find(|x| char::is_digit(x, 10)) {
                values.push((value, line.chars().nth(value).unwrap().to_string()))
            }

            if let Some(value) = line.rfind(|x| char::is_digit(x, 10)) {
                values.push((value, line.chars().nth(value).unwrap().to_string()))
            }

            values.sort_by(|a, b| {
                if a.0 < b.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            values
        })
        .map(|v| {
            let first_digit = if let Some(value) = numbers
                .iter()
                .position(|c| c == &v.first().expect("No first value").1)
            {
                value + 1
            } else {
                v.first().unwrap().1.parse().unwrap()
            };

            let last_digit = if let Some(value) = numbers
                .iter()
                .position(|c| c == &v.last().expect("No last value").1)
            {
                value + 1
            } else {
                v.last().unwrap().1.parse().unwrap()
            };

            format!("{first_digit}{last_digit}")
                .parse::<usize>()
                .expect("Failed to parse back to digit at index {n}")
        })
        .sum()
}
