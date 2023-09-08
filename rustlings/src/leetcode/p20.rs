use std::{collections::HashMap, vec};

fn is_balanced(mask: i32) -> bool {
    if mask == 0 {
        return false;
    }

    (mask & (mask - 1)) == 0
}

fn is_valid(s: String) -> bool {
    let mut mask = 0b000;

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => mask ^= 1 << 0,
            ')' => {
                if mask >> 0 & 1 == 1 {
                    mask ^= 1 << 0
                } else {
                    return false;
                }
            }
            '{' => mask ^= 1 << 1,
            '}' => {
                if mask >> 1 & 1 == 1 {
                    mask ^= 1 << 1
                } else {
                    return false;
                }
            }
            '[' => mask ^= 1 << 2,
            ']' => {
                if mask >> 2 & 1 == 1 {
                    mask ^= 1 << 2
                } else {
                    return false;
                }
            }
            _ => todo!(),
        };

        println!("{:b} {c}", mask);
    }

    mask == 0
}

fn is_valid_stack(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            ')' => {
                if stack.is_empty() || stack.pop().unwrap() != '(' {
                    return false;
                }
            }
            '}' => {
                if stack.is_empty() || stack.pop().unwrap() != '{' {
                    return false;
                }
            }
            ']' => {
                if stack.is_empty() || stack.pop().unwrap() != '[' {
                    return false;
                }
            }
            _ => stack.push(c),
        }
    }

    stack.is_empty()
}

#[test]
fn is_balanced_test() {
    assert_eq!(is_balanced(0), false);
    assert_eq!(is_balanced(1), true);
    assert_eq!(is_balanced(2), true);
    assert_eq!(is_balanced(3), false);
    assert_eq!(is_balanced(4), true);
    assert_eq!(is_balanced(6), false);
    assert_eq!(is_balanced(8), true);
}

#[test]
fn is_valid_test() {
    assert_eq!(is_valid_stack("()".to_string()), true);
    assert_eq!(is_valid_stack("()[]{}".to_string()), true);
    assert_eq!(is_valid_stack("(]".to_string()), false);
    assert_eq!(is_valid_stack("(])[".to_string()), false);
    assert_eq!(is_valid_stack("([]{})".to_string()), true);
    assert_eq!(is_valid_stack("[()]{}".to_string()), true);
    assert_eq!(is_valid_stack("([)]".to_string()), false);
}
