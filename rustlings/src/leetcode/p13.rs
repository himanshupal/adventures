pub fn roman_to_int(s: String) -> i32 {
    let mut value = 0i32;
    let mut length = s.len();

    while length > 0 {
        let it = s.len() - length;
        let char = s.chars().nth(it).unwrap();
        let next_char = if it < s.len() - 1 {
            s.chars().nth(it + 1).unwrap()
        } else {
            '*'
        };
        value += match char {
            'I' => {
                if next_char == 'V' {
                    length -= 2;
                    4
                } else if next_char == 'X' {
                    length -= 2;
                    9
                } else {
                    length -= 1;
                    1
                }
            }
            'V' => {
                length -= 1;
                5
            }
            'X' => {
                if next_char == 'L' {
                    length -= 2;
                    40
                } else if next_char == 'C' {
                    length -= 2;
                    90
                } else {
                    length -= 1;
                    10
                }
            }
            'L' => {
                length -= 1;
                50
            }
            'C' => {
                if next_char == 'D' {
                    length -= 2;
                    400
                } else if next_char == 'M' {
                    length -= 2;
                    900
                } else {
                    length -= 1;
                    100
                }
            }
            'D' => {
                length -= 1;
                500
            }
            'M' => {
                length -= 1;
                1000
            }
            _ => todo!(),
        }
    }
    value
}

#[test]
fn roman_to_int_test() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}
