use std::fs;

fn main() {
    if let Ok(data) = fs::read_to_string("../input.txt") {
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

        println!("Sum: {sum}")
    }
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
