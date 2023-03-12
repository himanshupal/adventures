use std::{fs, ops::Shl};

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {return};

    println!("Problem1: {}", get_marker(&data, 4));
    println!("Problem2: {}", get_marker(&data, 14));
}

fn get_marker(data: &str, pos_init: usize) -> usize {
    for i in 0..(data.len() - (pos_init - 1)) {
        let mut map: i32 = 0;
        let mut repeated = false;
        let slice = &data[i..(i + pos_init)];
        for ch in slice.chars() {
            let mask = 1.shl(ch as i32 - b'a' as i32);
            if map & mask != 0 {
                repeated = true;
                break;
            }
            map |= mask;
        }
        if !repeated {
            return i + pos_init;
        }
    }
    0
}
