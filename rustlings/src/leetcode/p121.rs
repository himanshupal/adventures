pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut pair_found = false;
    let mut last_min = i32::MAX;
    let (mut min, mut max) = (i32::MAX, i32::MIN);

    prices.into_iter().enumerate().for_each(|(index, price)| {
        if price <= min {
            last_min = min;
            min = price;
            pair_found = false;
        } else if price >= max {
            max = price;
            pair_found = true;
        }
    });

    println!("{min} {max} {last_min} {pair_found}");

    match min != i32::MAX && max != i32::MIN {
        true => match pair_found {
            true => max - min,
            false => {
                max - match last_min != i32::MIN {
                    true => last_min,
                    false => min,
                }
            }
        },
        false => 0,
    }
}

#[test]
fn max_profit_test() {
    // assert_eq!(max_profit(vec![2, 4, 1]), 2);
    // assert_eq!(max_profit(vec![2, 1, 2, 1, 0, 1, 2]), 2);
    assert_eq!(max_profit(vec![2, 1, 2, 1, 0, 0, 1]), 1);
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!( 
        max_profit(vec![5, 3, 1, 8, 3, 8, 5, 6, 1, 1, 3, 12, 7, 9]),
        11
    );
}
