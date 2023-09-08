pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].to_string();
    }

    let mut prefix = String::new();

    for i in 0..(strs.len() - 1) {
        let mut min_length = {
            let cl = (if i == 0 { &strs[i] } else { &prefix }).len();
            let nl = strs[i + 1].len();
            if cl < nl {
                cl
            } else {
                nl
            }
        };

        if min_length == 0 {
            return String::new();
        }

        if prefix.len() > min_length {
            prefix = prefix[..min_length].to_string();
        }

        for x in 0..min_length {
            let s = if i == 0 { &strs[i] } else { &prefix };
            if &s.chars().nth(x) == &strs[i + 1].chars().nth(x) {
                if i == 0 {
                    prefix = format!("{prefix}{}", &strs[i].chars().nth(x).unwrap());
                }
            } else if x == 0 {
                return String::new();
            } else if i == 0 {
                break;
            } else if prefix.len() > 0 {
                prefix = prefix[..prefix.len() - 1].to_string();
            }
        }
    }

    prefix
}

pub fn longest_common_prefix_simple(strs: Vec<String>) -> String {
    let min_length = strs.iter().map(|s| s.len()).min().unwrap();
    let mut max_length = 0;

    'outer: for i in 0..min_length {
        for (x, v) in strs[..strs.len() - 1].iter().enumerate() {
            if v.chars().nth(i) != strs[x + 1].chars().nth(i) {
                break 'outer;
            }
        }

        max_length += 1;
    }

    strs[0][..max_length].to_string()
}

pub fn longest_common_prefix_window(strs: Vec<String>) -> String {
    let mut n = strs.iter().map(|s| s.len()).min().unwrap();
    while strs.windows(2).any(|s| s[0][..n] != s[1][..n]) {
        n -= 1;
    }
    strs[0][..n].to_string()
}

#[test]
fn longest_common_prefix_test() {
    assert_eq!(
        longest_common_prefix(
            vec!["flower", "flow", "flight"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        "fl"
    );
    assert_eq!(
        longest_common_prefix(
            vec!["dog", "racecar", "car"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        ""
    );
    assert_eq!(
        longest_common_prefix(
            vec!["human", "height", "plant"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        ""
    );
    assert_eq!(
        longest_common_prefix(
            vec!["plain", "pl", "xa"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        ""
    );
    assert_eq!(
        longest_common_prefix(
            vec!["pl", "plane", "plate"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        "pl"
    );
    assert_eq!(
        longest_common_prefix(
            vec!["a"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        "a"
    );
    assert_eq!(
        longest_common_prefix(
            vec!["cir", "car"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        "c"
    );
    assert_eq!(
        longest_common_prefix(
            vec!["abab", "aba", ""]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        ""
    );
    assert_eq!(
        longest_common_prefix(
            vec!["cb", "cbb", "a"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        ""
    );
    assert_eq!(
        longest_common_prefix(
            vec!["ac", "ac", "a", "a"]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        ),
        "a"
    );
}
