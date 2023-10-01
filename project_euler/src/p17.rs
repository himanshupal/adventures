use std::time::Instant;

fn get_single_char_count(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 | 6 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 => 5,
        _ => unreachable!(),
    }
}

fn get_two_char_count(n: usize) -> usize {
    match n / 10 {
        1 => match n {
            10 => 3,
            11 | 12 => 6,
            15 | 16 => 7,
            13 | 14 | 18 | 19 => 8,
            17 => 9,
            _ => unreachable!(),
        },
        2 | 3 | 8 | 9 => 6 + get_single_char_count(n % 10),
        4 | 5 | 6 => 5 + get_single_char_count(n % 10),
        7 => 7 + get_single_char_count(n % 10),
        _ => get_single_char_count(n % 10),
    }
}

fn get_three_char_count(n: usize) -> usize {
    match n % 100 {
        0 => get_single_char_count(n / 100) + 7,
        _ => get_single_char_count(n / 100) + 7 + 3 + get_two_char_count(n % 100),
    }
}

fn get_count(n: usize) -> usize {
    match n {
        1000 => 11,
        x if x / 100 > 0 => get_three_char_count(n),
        x if x / 10 > 0 => get_two_char_count(n),
        _ => get_single_char_count(n),
    }
}

pub fn letter_count_sum(max: usize) {
    let now = Instant::now();
    println!(
        "17: Found after {} seconds: {}",
        now.elapsed().as_secs(),
        (1..=max).map(get_count).sum::<usize>()
    )
}
