use std::{collections::BTreeSet, time::Instant};

use crate::utils::get_factors_sum;

const LIMIT: usize = 28123;

fn get_abundant_numbers(limit_ex: usize) -> BTreeSet<usize> {
    (1..limit_ex).fold(BTreeSet::new(), |mut p, number| {
        if get_factors_sum(number) > number {
            p.insert(number);
        }
        p
    })
}

pub fn non_abundant_sums() {
    let now = Instant::now();
    let abundant_numbers = get_abundant_numbers(LIMIT);

    let not_sum_of_abundants = (1..LIMIT).fold(vec![], |mut p, number| {
        let mut skip_push = false;
        let mut it = abundant_numbers.iter();
        let mut next_value = *it.next().unwrap();

        while next_value < number {
            if !abundant_numbers.contains(&(number - next_value)) {
                next_value = *it.next().unwrap();
            } else {
                skip_push = true;
                break;
            }
        }

        if !skip_push {
            p.push(number);
        }

        p
    });

    println!(
        "23: Found after {} seconds: {:?}",
        now.elapsed().as_secs(),
        not_sum_of_abundants.iter().sum::<usize>()
    )
}
