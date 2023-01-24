use std::{fs, process, usize};

fn main() {
    if let Ok(data) = fs::read_to_string("../input.txt") {
        let group_sums = data.split("\n\n").map(|group| {
            group
                .split("\n")
                .map(|value| value.parse().unwrap_or(0))
                .sum()
        });

        let mut g = group_sums.collect::<Vec<usize>>();
        let last_index = g.len() - 1;
        g.sort();

        println!("Highest is {}", g[last_index]);
    } else {
        eprintln!("Error loading file...");
        process::exit(0);
    }
}
