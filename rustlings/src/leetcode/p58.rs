pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().nth_back(0).unwrap().len() as i32
}

pub fn length_of_last_word_alt(s: String) -> i32 {
    let mut index = 0;
    let mut length = 0;
    let mut chars = s.chars();
    loop {
        match chars.nth_back(index) {
            Some(c) if c == ' ' => {
                if length != 0 {
                    length = 0;
                }
            }
            _ => length += 1,
        };
        index += 1;
    }
    length
}

#[test]
fn length_of_last_word_test() {
    assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(
        length_of_last_word("   fly me   to   the moon  ".to_string()),
        4
    );
    assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
}
