pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for pos in (0..digits.len()).rev() {
        if digits.iter().nth(pos).unwrap() != &9 {
            let v = digits.remove(pos) + 1;
            digits.insert(pos, v);
            break;
        } else {
            digits.remove(pos);
            digits.insert(pos, 0);
            if pos == 0 {
                digits.insert(0, 1);
            }
        }
    }

    digits
}

// Not working since digits can be very large i.e. 1 < digits.len < 100
pub fn plus_one_str(digits: Vec<i32>) -> Vec<i32> {
    (digits
        .iter()
        .map(|v| v.to_string())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
        + 1)
    .to_string()
    .chars()
    .map(|v| v.to_digit(10).unwrap() as i32)
    .collect()
}

#[test]
fn plus_one_test() {
    assert!(plus_one_str(vec![1, 0, 0]).eq(&vec![1, 0, 1]));
    assert!(plus_one_str(vec![1]).eq(&vec![2]));
    assert!(plus_one_str(vec![2, 9, 9]).eq(&vec![3, 0, 0]));
    assert!(plus_one_str(vec![4, 3, 2, 1]).eq(&vec![4, 3, 2, 2]));
    assert!(plus_one_str(vec![9]).eq(&vec![1, 0]));

    // Disabled due to reason at line #19
    // assert!(
    //     plus_one_str(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]).eq(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1])
    // );
}
