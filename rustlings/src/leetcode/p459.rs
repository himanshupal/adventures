pub fn repeated_substring_pattern(s: String) -> bool {
    let length = s.len();
    for i in 0..(length / 2) {
        println!("Slice: {} | Repeating {} times", &s[..=i], length / (i + 1));

        if &s[..=i].repeat(length / (i + 1)) == &s {
            return true;
        }
    }

    false
}

#[test]
fn repeated_substring_pattern_test() {
    assert!(!repeated_substring_pattern(String::from("a")));
    assert!(repeated_substring_pattern(String::from("bb")));
    assert!(repeated_substring_pattern(String::from("abab")));
    assert!(!repeated_substring_pattern(String::from("aba")));
    assert!(repeated_substring_pattern(String::from("abcabcabcabc")));
}
