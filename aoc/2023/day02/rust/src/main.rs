use std::{collections::HashMap, fs};

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {
        panic!("Couldn't read input file");
    };

    println!("Part 1: {}", part1(&data));
}

fn part1(data: &String) -> usize {
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 15)]);

    data.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let sets = line
                .split([':'])
                .nth(1)
                .unwrap()
                .split([';'])
                .map(|set| set.split([',']).map(|game| game.trim()).collect())
                .collect::<Vec<Vec<&str>>>();

            let mut is_bad = false;
            'outer: for set in sets.iter() {
                for game in set.iter() {
                    let mut splitted = game.split_whitespace();
                    let count: i32 = splitted.next().unwrap().trim().parse().unwrap();
                    let color = splitted.next().unwrap();

                    if let Some(limit) = limits.get(color) {
                        if &count > limit {
                            is_bad = true;
                            break 'outer;
                        }
                    }
                }
            }

            if !is_bad {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}
