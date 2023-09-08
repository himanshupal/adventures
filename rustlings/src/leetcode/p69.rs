pub fn my_sqrt(x: i32) -> i32 {
    let (mut current, mut next) = (0i64, 1i64);
    let n = i64::from(x);

    loop {
        if n >= next * next {
            current = next;
            next += 1;
        } else {
            break;
        }
    }

    current as i32
}

#[test]
fn my_sqrt_test() {
    assert_eq!(my_sqrt(0), 0);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(2), 1);
    assert_eq!(my_sqrt(3), 1);
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(5), 2);
    assert_eq!(my_sqrt(7), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(9), 3);
    assert_eq!(my_sqrt(144), 12);
    assert_eq!(my_sqrt(2147395600), 46340);
}
