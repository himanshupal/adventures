use std::time::Instant;

fn all_factors_sum(number: usize) -> usize {
    (1..=number / 2)
        .fold(vec![], |mut p, c| {
            if number % c == 0 {
                p.push(c);
            }
            p
        })
        .iter()
        .sum()
}

pub fn amicable_numbers() {
    let now = Instant::now();
    let mut amicables = vec![];

    for n in 1..10000 {
        let first_sum = all_factors_sum(n);
        if n != first_sum && n == all_factors_sum(first_sum) {
            amicables.push(n);
        }
    }

    println!(
        "21: Found after {} seconds: {}",
        now.elapsed().as_secs(),
        amicables.iter().sum::<usize>()
    )
}
