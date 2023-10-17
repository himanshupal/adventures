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
