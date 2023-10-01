use std::{fs, time::Instant};

pub fn names_scores() {
    let now = Instant::now();
    let data = fs::read_to_string("inputs/p22").expect("Couldn't read input file");

    let mut names = data
        .lines()
        .nth(0)
        .expect("Empty file")
        .split(&[',', '"'])
        .filter(|p| !p.is_empty())
        .collect::<Vec<&str>>();

    names.sort();

    println!(
        "22: Found after {} milliseconds: {}",
        now.elapsed().as_millis(),
        names
            .iter()
            .enumerate()
            .map(|(p, name)| { (p as u32 + 1) * name.chars().fold(0, |p, c| p + c as u32 - 64) })
            .sum::<u32>()
    )
}
