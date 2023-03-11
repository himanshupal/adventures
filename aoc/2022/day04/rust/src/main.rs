// use regex::Regex;
use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {return};

    let sum = data
        .lines()
        .map(|t| {
            let mut contains = 0;
            let mut previous = (0, 0);
            t.split(",").for_each(|range| {
                let values: Vec<&str> = range.split("-").collect();
                let (v1, v2): (u32, u32) = (values[0].parse().unwrap(), values[1].parse().unwrap());
                if previous.0 == 0 && previous.1 == 0 {
                    previous = (v1, v2);
                } else if (previous.0 <= v1 && previous.1 >= v2) || (previous.0 >= v1 && previous.1 <= v2) {
                    contains = 1;
                }
            });
            contains
        })
        .sum::<u32>();

    /* let sum_with_regex = data
    .lines()
    .map(|t| {
        let r = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        for group in r.captures_iter(t) {
            let (l1, h1): (u32, u32) = (&group[1].parse().unwrap(), &group[2].parse().unwrap());
            let (l2, h2): (u32, u32) = (&group[3].parse().unwrap(), &group[4].parse().unwrap());
            if (l1 <= l2 && h1 >= h2) || (l1 >= l2 && h1 <= h2) {
                return 1;
            }
        }
        0
    })
    .sum::<u32>(); */

    println!("Sum: {sum}");

    let sum = data
        .lines()
        .map(|t| {
            let mut contains = 0;
            let mut previous = (0, 0);
            t.split(",").for_each(|range| {
                let values: Vec<&str> = range.split("-").collect();
                let (v1, v2): (u32, u32) = (values[0].parse().unwrap(), values[1].parse().unwrap());
                if previous.0 == 0 && previous.1 == 0 {
                    previous = (v1, v2);
                } else if previous.0 <= v2 && previous.1 >= v1 {
                    // https://stackoverflow.com/a/3269471
                    contains = 1;
                }
            });
            contains
        })
        .sum::<u32>();

    println!("Part2: {sum}")
}
