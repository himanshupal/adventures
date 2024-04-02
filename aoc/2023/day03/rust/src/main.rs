use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {
        panic!("Couldn't read input file");
    };

    let values: Vec<HashMap<(usize, usize), usize>> = data
        .lines()
        .map(|line| {
            let mut start_index = 0;
            let mut read_in_progress = false;
            let mut lut = HashMap::new();

            let get_value = |start_index: &usize, index: &usize| {
                let number_as_str = &line.chars().collect::<Vec<char>>()[*start_index..*index]
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(&String::new());
                number_as_str
                    .parse::<usize>()
                    .expect(&format!("Failed to parse {number_as_str} to number"))
            };

            line.chars().enumerate().for_each(|(index, ch)| {
                match (read_in_progress, ch.is_alphanumeric()) {
                    (false, true) => {
                        read_in_progress = true;
                        start_index = index;
                    }
                    (true, false) => {
                        read_in_progress = false;
                        lut.insert((start_index, index - 1), get_value(&start_index, &index));
                    }
                    _ => (),
                }
            });

            if read_in_progress {
                lut.insert(
                    (start_index, (line.len() - 1)),
                    get_value(&start_index, &line.len()),
                );
            }

            lut
        })
        .collect();

    println!("Part 1: {}", part_01(&data, &values));
}

fn get_nearby_pos(x: usize, y: usize, x_limit: usize, y_limit: usize) -> Vec<(usize, usize)> {
    let mut list = vec![];

    match (x, y) {
        (0, 0) => {}
        _ => match (x, y) {
            (0, _) => {
                list.push((x, y - 1));

                if x != x_limit {
                    list.push((x + 1, y - 1));
                }

                if y != y_limit {
                    list.push((x, y + 1));
                }

                if x != x_limit && y != y_limit {
                    list.push((x + 1, y + 1));
                }
            }
            (_, 0) => {
                list.push((x - 1, y));

                if x != x_limit {
                    list.push((x + 1, y));
                }

                if y != y_limit {
                    list.push((x - 1, y + 1));
                }

                if x != x_limit && y != y_limit {
                    list.push((x + 1, y + 1));
                }
            }
            _ => {
                list.push((x, y - 1));
                list.push((x - 1, y));
                list.push((x - 1, y - 1));

                if x != x_limit {
                    list.push((x + 1, y));
                    list.push((x + 1, y - 1));
                }

                if y != y_limit {
                    list.push((x, y + 1));
                    list.push((x - 1, y + 1));
                }

                if x != x_limit && y != y_limit {
                    list.push((x + 1, y + 1));
                }
            }
        },
    }

    list
}

fn part_01(data: &String, values: &Vec<HashMap<(usize, usize), usize>>) -> usize {
    let mut parts_found = HashSet::new();

    data.lines().enumerate().for_each(|(index, line)| {
        for (i, char) in line.chars().enumerate() {
            if char == '.' || char.is_alphanumeric() {
                continue;
            }

            for point in get_nearby_pos(index, i, line.len() - 1, data.lines().count() - 1).iter() {
                for (index, map) in values.iter().enumerate() {
                    if point.0 != index || map.is_empty() {
                        continue;
                    }

                    for k in map.keys() {
                        if point.1 >= k.0 && point.1 <= k.1 {
                            parts_found.insert((index, i, map.get(&k).unwrap()));
                        }
                    }
                }
            }
        }
    });

    parts_found.iter().map(|x| x.2).sum()
}
