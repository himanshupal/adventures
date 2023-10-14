use std::{fs, time::Instant};

fn route_sum(data: &Vec<Vec<usize>>, from_line: usize, from_pos: usize, prev_sum: usize) -> usize {
    if from_line < data.len() {
        let max = (from_pos..=from_pos + 1)
            .map(|v| {
                if v < data[from_line].len() {
                    route_sum(data, from_line + 1, v, data[from_line][v] + prev_sum)
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);
        if max > prev_sum {
            return max;
        }
    }

    prev_sum
}

pub fn max_path_sum() {
    let now = Instant::now();

    let data = fs::read_to_string("inputs/p18")
        .expect("Couldn't read input file")
        .lines()
        .fold(vec![], |mut p: Vec<Vec<usize>>, line: &str| {
            p.push(
                line.split_whitespace()
                    .map(|value| {
                        value
                            .parse::<usize>()
                            .expect(&format!("{value} isn't a number"))
                    })
                    .collect(),
            );
            p
        });

    println!(
        "18: Found after {} microseconds: {}",
        now.elapsed().as_micros(),
        route_sum(&data, 0, 0, 0)
    )
}
