use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {
        return;
    };

    let matches = data
        .lines()
        .map(|line| {
            line.split(&[':', '|'])
                .enumerate()
                .filter_map(|(index, words)| {
                    if index != 0 {
                        Some(words.split_whitespace().collect())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Vec<&str>>>()
        })
        .map(|list| {
            let found: Vec<&str> = list[1]
                .iter()
                .filter_map(|value| {
                    if list[0].contains(value) {
                        Some(*value)
                    } else {
                        None
                    }
                })
                .collect();

            if found.len() > 0 {
                2u32.pow((found.len() - 1) as u32)
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("Matches: {:?}", matches);
}
