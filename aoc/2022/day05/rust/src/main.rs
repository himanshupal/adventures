use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {return};

    let mut line_counter = 0;
    let mut string = String::new();
    let mut moves: Vec<(u32, u32, u32)> = Vec::new();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    data.lines().for_each(|line| {
        if line_counter < 8 {
            line_counter += 1;
            let chars: Vec<char> = line.chars().collect();
            (0..chars.len()).step_by(4).for_each(|i| {
                let ch = chars[i + 1];
                if ch != ' ' {
                    stacks[(f32::floor((i / 4) as f32)) as usize].insert(0, ch);
                }
            });
        } else if line_counter < 10 {
            line_counter += 1
        } else {
            let words: Vec<&str> = line.split_whitespace().collect();
            moves.push((
                words[1].parse().unwrap(),
                words[3].parse().unwrap(),
                words[5].parse().unwrap(),
            ));
        };
    });

    moves.iter().for_each(|(count, from, to)| {
        for _ in 0..count.to_owned() {
            let top = stacks[(from.to_owned() - 1) as usize].pop().unwrap();
            stacks[(to.to_owned() - 1) as usize].push(top);
        }
    });

    stacks.iter().for_each(|stack| {
        string.push(*stack.last().unwrap());
    });

    println!("{string}");
}
