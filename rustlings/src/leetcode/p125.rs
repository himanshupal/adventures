pub fn is_palindrome(s: String) -> bool {
    let filtered: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric() && *c != ' ')
        .collect();

    for (i, c) in filtered.iter().rev().enumerate() {
        if (*c).to_lowercase().ne(filtered[i].to_lowercase()) {
            return false;
        }
    }

    true
}

#[test]
fn is_palindrome_test() {
    assert!(!is_palindrome(String::from("A B 5:s")));
    assert!(is_palindrome(String::from(
        "A man, a plan, a canal: Panama"
    )));
    assert!(!is_palindrome(String::from("race a car")));
    assert!(is_palindrome(String::from(" ")));
}
