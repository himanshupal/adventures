use std::{fs, ops::Shl};

fn main() {
    let Ok(data) = fs::read_to_string("../input.txt") else {return};
    let mut pos = 0;

    for i in 0..(data.len() - 3) {
        let mut map: i32 = 0;
        let mut repeated = false;
        let slice = &data[i..i + 4];
        for ch in slice.chars() {
            let mask = 1.shl(ch as i32 - b'a' as i32);
            if map & mask != 0 {
                repeated = true;
                break;
            }
            map |= mask;
        }
        if !repeated {
            pos = i + 4;
            break;
        }
    }

    println!("Problem1: {pos}");
}
