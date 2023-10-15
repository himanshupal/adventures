pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|n| match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => n.to_string(),
        })
        .collect()
}

#[test]
fn fizz_buzz_test() {
    assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
    assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    assert_eq!(
        fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
    );
}
