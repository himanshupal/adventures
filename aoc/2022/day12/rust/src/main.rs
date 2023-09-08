use collections::{HashMap, HashSet};
use std::{collections, fs};

fn main() {
    let Ok(data) = fs::read_to_string("../_input.txt") else { return } ;

    let mut graph = HashMap::new();
    let num_of_lines = data.lines().count();
    let num_of_chars = data.lines().nth(0).expect("Failed reading chars for line 0").chars().count();

    data.lines().enumerate().for_each(|(line_no, line)| {
        let chars = line.chars();
        chars.clone().enumerate().for_each(|(char_no, char)| {
            let mut neighbors = HashSet::new();

            if char_no > 0 {
                neighbors.insert(chars.clone().nth(char_no - 1).expect(&format!("Couldn't read char at {char_no}")));
            }
            if char_no < num_of_chars - 1 {
                neighbors.insert(chars.clone().nth(char_no + 1).expect(&format!("Couldn't read char at {char_no}")));
            }
            if line_no > 0 {
                neighbors.insert(
                    data.lines()
                        .nth(line_no - 1)
                        .expect(&format!("Couldn't read line {line_no}"))
                        .chars()
                        .nth(char_no)
                        .expect(&format!("Couldn't read char at {char_no}")),
                );
            }
            if line_no < num_of_lines - 1 {
                neighbors.insert(
                    data.lines()
                        .nth(line_no + 1)
                        .expect(&format!("Couldn't read line {line_no}"))
                        .chars()
                        .nth(char_no)
                        .expect(&format!("Couldn't read char at {char_no}")),
                );
            }

            graph.insert(char, neighbors);
        });
    });

    println!("{:?}", graph);
}
