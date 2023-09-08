pub fn reverse(x: i32) -> i32 {
    let mut reverse = 0;

    {
        let mut temp = i32::abs(x);
        while temp > 0 {
            if let Some(v) = 10i32.checked_mul(reverse) {
                reverse = v + (temp % 10);
                temp /= 10;
            } else {
                return 0;
            }
        }
    }

    reverse * if x.is_negative() { -1 } else { 1 }
}

#[test]
fn reverse_test() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(1534236469), 0);
    assert_eq!(reverse(i32::MAX), 0);
}
