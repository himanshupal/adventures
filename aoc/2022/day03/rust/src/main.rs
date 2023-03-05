use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else { return };

    let sum = data
        .lines()
        .map(|text| {
            let text_length = text.len();
            let half_length = text_length / 2;
            let first_part = &text[0..half_length];
            let second_part = &text[half_length..text_length];

            for char in first_part.chars() {
                if second_part.contains(char) {
                    return get_weight(char);
                }
            }

            0
        })
        .sum::<u32>();

    let lines = data.lines().collect::<Vec<&str>>();
    let mut it = lines.chunks(3);
    let mut sum_part2 = 0;

    loop {
        let Some(broken_lines) = it.next() else { break };

        let first_line = broken_lines.get(0).unwrap();
        let second_line = broken_lines.get(1).unwrap();
        let third_line = broken_lines.get(2).unwrap();

        for char in first_line.chars() {
            if second_line.contains(char) {
                if third_line.contains(char) {
                    sum_part2 += get_weight(char);
                    break; // Only need to be read once
                }
            }
        }
    }

    println!("Sum: {sum} Part2: {sum_part2}");
}

fn get_weight(ch: char) -> u32 {
    let digit = u32::from(ch);

    if digit >= 65 && digit <= 65 + 26 {
        digit - 65 + 27
    } else if digit >= 97 && digit <= 97 + 26 {
        digit - 97 + 1
    } else {
        0
    }
}
