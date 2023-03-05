use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {return};

    let sum = data
        .lines()
        .map(|t| {
            let mut contains = 0;
            let mut previous = (0, 0);
            t.split(",").for_each(|range| {
                let values = range.split("-").collect::<Vec<&str>>();
                let (v1, v2) = (u32::from(values[0].parse::<u32>().unwrap()), u32::from(values[1].parse::<u32>().unwrap()));
                if previous.0 == 0 && previous.1 == 0 {
                    previous = (v1, v2);
                } else {
                    if (previous.0 <= v1 && previous.1 >= v2) || (previous.0 >= v1 && previous.1 <= v2) {
                        contains = 1;
                    }
                }
            });
            contains
        })
        .sum::<u32>();

    println!("Sum: {sum}")
}
