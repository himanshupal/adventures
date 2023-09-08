pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        index as i32
    } else {
        -1
    }
}

pub fn str_str_window(haystack: String, needle: String) -> i32 {
    let n_chars = &needle.chars().collect::<Vec<char>>();

    for (i, m) in haystack
        .chars()
        .collect::<Vec<_>>()
        .windows(needle.len())
        .enumerate()
    {
        if m.eq(n_chars) {
            return i as i32;
        }
    }

    -1
}

#[test]
fn str_str_test() {
    assert_eq!(
        str_str_window("sadbutsad".to_string(), "sad".to_string()),
        0
    );
    assert_eq!(
        str_str_window("leetcode".to_string(), "leeto".to_string()),
        -1
    );
}
