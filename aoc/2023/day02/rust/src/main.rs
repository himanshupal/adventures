use std::{collections::HashMap, fs};

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {
        panic!("Couldn't read input file");
    };

    let parsed_data: Vec<(usize, Vec<Vec<(usize, &str)>>)> = data
        .lines()
        .enumerate()
        .map(|(index, line)| {
            (
                index,
                line.split([':'])
                    .nth(1)
                    .unwrap()
                    .split([';'])
                    .map(|set| set.split([',']).map(|game| game.trim()))
                    .map(|set| {
                        set.map(|game| {
                            let mut splitted = game.split_whitespace();
                            let count: usize = splitted.next().unwrap().trim().parse().unwrap();
                            let color = splitted.next().unwrap();
                            (count, color)
                        })
                        .collect()
                    })
                    .collect(),
            )
        })
        .collect();

    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

fn part1(data: &Vec<(usize, Vec<Vec<(usize, &str)>>)>) -> usize {
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 15)]);

    data.iter()
        .filter_map(|(index, set)| {
            for game in set {
                for (count, color) in game {
                    if let Some(limit) = limits.get(color) {
                        if count > limit {
                            return None;
                        }
                    }
                }
            }
            Some(index + 1)
        })
        .sum()
}

fn part2(data: &Vec<(usize, Vec<Vec<(usize, &str)>>)>) -> usize {
    data.iter()
        .map(|(_, set)| {
            let mut max_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for game in set {
                for (count, color) in game {
                    if max_map.get(color).unwrap() < count {
                        max_map.insert(color, *count);
                    }
                }
            }
            max_map.values().product::<usize>()
        })
        .sum()
}
