use std::time::Instant;

use crate::utils::factorial;

pub fn lexographic_permutations() {
    let now = Instant::now();

    let mut input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut remainder = 1_000_000 - 1;
    let mut it = input.len() - 1;
    let mut answer = vec![];

    while input.len() > 0 {
        let factorial = factorial(it);
        let value = remainder / factorial;
        remainder %= factorial;

        answer.push(input[value]);
        input.remove(value);

        if it > 0 {
            it -= 1;
        }
    }

    println!(
        "24: Found after {} milliseconds: {}",
        now.elapsed().as_millis(),
        answer.iter().map(|i| i.to_string()).collect::<String>()
    )
}
