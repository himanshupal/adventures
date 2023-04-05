use std::fs;

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else { return };

    let lines = data.lines().collect::<Vec<&str>>();

    let no_of_lines = lines.len();
    let last_line_index = no_of_lines - 1;

    let text_length = lines[0].len();
    let last_char_index = text_length - 1;

    let mut visibility = vec![0u128; no_of_lines];

    let mut max_score = 0;

    for (line_number, line) in data.lines().enumerate() {
        if line_number == 0 || line_number == last_line_index {
            visibility[line_number] |= (1 << text_length) - 1;
            continue;
        }

        for (index, ch) in line.chars().enumerate() {
            let underlying = ch.to_digit(10).expect("NaN");

            if index == 0 || index == last_char_index {
                visibility[line_number] |= 1 << index;
                continue;
            }

            let mut visible = 0;
            let (mut t, mut r, mut b, mut l) = (0, 0, 0, 0);

            for distance in 0..no_of_lines {
                let next_location = distance + 1;

                // println!(">>> {line_number}:{index}:{distance}:{underlying}");

                if (visible & (1 << 3)) == 0 && line_number >= next_location && next_location < last_line_index {
                    let value = get_char_at_index(lines[line_number - next_location], index);
                    // println!("T {value}");
                    if value >= underlying {
                        visible |= 1 << 3;
                    }
                    t += 1;
                }

                if (visible & (1 << 2)) == 0 && next_location > index && next_location <= last_char_index {
                    let value = get_char_at_index(lines[line_number], next_location);
                    // println!("R {value}");
                    if value >= underlying {
                        visible |= 1 << 2;
                    }
                    r += 1;
                }

                if (visible & (1 << 1)) == 0 && line_number < distance {
                    let value = get_char_at_index(lines[distance], index);
                    // println!("B {value}");
                    if value >= underlying {
                        visible |= 1 << 1;
                    }
                    b += 1;
                }

                if (visible & (1 << 0)) == 0 && index >= next_location {
                    let value = get_char_at_index(lines[line_number], index - next_location);
                    // println!("L {value}");
                    if value >= underlying {
                        visible |= 1 << 0;
                    }
                    l += 1;
                }
            }

            if visible != 0xF {
                visibility[line_number] |= 1 << (last_char_index - index);
            }

            let score = t * r * b * l;

            if score > max_score {
                max_score = score;
            }
        }
    }

    // for row in visibility.clone() {
    //     println!("{:b}", row);
    // }

    println!("part_01: {:?}, part_02: {max_score}", visibility.iter().map(|m| count_bits(*m)).sum::<u128>());
}

fn get_char_at_index(text: &str, index: usize) -> u32 {
    text.chars().nth(index).expect("unreachable index").to_digit(10).expect("NaN")
}

fn count_bits(mut value: u128) -> u128 {
    /* Works only for 32 bit int */
    // let count = value - ((value >> 1) & 033333333333) - ((value >> 2) & 011111111111);
    // ((count + (count >> 3)) & 030707070707) % 63

    let mut count = 0;
    while value != 0 {
        value &= value - 1;
        count += 1;
    }
    count
}
