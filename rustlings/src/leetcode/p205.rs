use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map = HashMap::new();

    if s.len() == 1 {
        return s == t;
    }

    let mut all_new = true;

    for (i, c) in s.chars().enumerate() {
        if let Some(ch) = map.get(&c) {
            if ch != &t.chars().nth(i).unwrap() {
                all_new = false;
                return false;
            }
        } else {
            map.insert(c, t.chars().nth(i).unwrap());
        }
    }

    all_new || map.len() != s.len()
}

#[test]
fn is_isomorphic_test() {
    assert!(is_isomorphic(String::from("ab"), String::from("ab")));
    assert!(is_isomorphic(String::from("a"), String::from("a")));
    assert!(is_isomorphic(String::from("egg"), String::from("add")));
    assert!(!is_isomorphic(String::from("foo"), String::from("bar")));
    assert!(is_isomorphic(String::from("paper"), String::from("title")));
    assert!(!is_isomorphic(String::from("badc"), String::from("baba")));
}
