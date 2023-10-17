use fnv::FnvHashMap;
use std::collections::BTreeMap;

pub fn get_factors(number: usize) -> Vec<usize> {
    ((1..=number / 2).fold(vec![], |mut p, c| {
        if number % c == 0 {
            p.push(c);
        }
        p
    }))
}

pub fn get_factors_sum(number: usize) -> usize {
    get_factors(number).iter().sum()
}

pub fn get_factors_cached(number: usize, cache: &mut BTreeMap<usize, Vec<usize>>) -> Vec<usize> {
    if let Some(factors) = cache.get(&number) {
        return factors.to_owned();
    }

    let factors = get_factors(number);
    cache.insert(number, factors.to_owned());
    return factors;
}

pub fn get_factors_sum_cached(number: usize, cache: &mut BTreeMap<usize, Vec<usize>>) -> usize {
    get_factors_cached(number, cache).iter().sum()
}

pub fn factorial(number: usize) -> usize {
    match number {
        0 => 1,
        _ => number * factorial(number - 1),
    }
}

pub fn nth_fibonacci(number: usize) -> usize {
    if number >= 2 {
        nth_fibonacci(number - 2) + nth_fibonacci(number - 1)
    } else {
        number
    }
}

pub fn nth_fibonacci_cached(number: usize, cache: &mut FnvHashMap<usize, usize>) -> usize {
    if let Some(value) = cache.get(&number) {
        return *value;
    }

    if number >= 2 {
        let value =
            nth_fibonacci_cached(number - 2, cache) + nth_fibonacci_cached(number - 1, cache);
        cache.insert(number, value);
        return value;
    } else {
        number
    }
}
