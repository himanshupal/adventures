use std::{collections::BTreeMap, time::Instant};

use crate::utils::get_factors_sum_cached;

pub fn amicable_numbers() {
    let now = Instant::now();
    let mut amicables = vec![];
    let cache = &mut BTreeMap::new();

    for n in 1..10000 {
        let first_sum = get_factors_sum_cached(n, cache);
        if n != first_sum && n == get_factors_sum_cached(first_sum, cache) {
            amicables.push(n);
        }
    }

    println!(
        "21: Found after {} micorseconds: {}",
        now.elapsed().as_micros(),
        amicables.iter().sum::<usize>()
    )
}
