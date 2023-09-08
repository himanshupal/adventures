use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s_length = s.len();

    if s_length > 1 {
        let mut max_length = 0;

        for group_length in 1..s_length {
            let mut seen_length = 0;

            let characters: Vec<char> = s.chars().into_iter().collect();
            characters.windows(group_length).for_each(|w| {
                println!("Window: {:?}", w);
                let mut local = HashSet::new();

                w.iter().for_each(|c| {
                    local.insert(c);
                });

                seen_length = seen_length.max(local.len());
            });

            println!("Seen length: {seen_length} Max length: {max_length}");

            if seen_length == group_length && seen_length > max_length {
                max_length = seen_length;
                println!("Updated max_length {max_length}");
            }
        }

        max_length as i32
    } else {
        s_length as i32
    }
}

#[test]
fn length_of_longest_substring_test() {
    assert_eq!(length_of_longest_substring(String::from(" ")), 1);
    assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
}
