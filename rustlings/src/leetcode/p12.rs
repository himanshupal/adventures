pub fn int_to_roman(mut num: i32) -> String {
    let mut roman = String::new();
    let map = vec![
        (1000, 1000, 'M'),
        (500, 100, 'D'),
        (100, 100, 'C'),
        (50, 10, 'L'),
        (10, 10, 'X'),
        (5, 1, 'V'),
        (1, 1, 'I'),
    ];

    for (v, d, c) in map {
        let times = num / d;

        println!("Before: {num} -> {roman}");
        println!("{v} * {times}");

        if num / v > 0 {
            match times * d {
                900 => {
                    println!("Matched: 900");
                    roman.push_str("CM");
                    num %= 100;
                }
                400 => {
                    println!("Matched: 400");
                    roman.push_str("CD");
                    num %= 100;
                }
                90 => {
                    println!("Matched: 90");
                    roman.push_str("XC");
                    num %= 10;
                }
                40 => {
                    println!("Matched: 40");
                    roman.push_str("XL");
                    num %= 10;
                }
                9 => {
                    println!("Matched: 9");
                    roman.push_str("IX");
                    break;
                }
                4 => {
                    println!("Matched: 4");
                    roman.push_str("IV");
                    break;
                }
                _ => match c {
                    'I' | 'X' | 'C' | 'M' => {
                        for i in 0..times {
                            println!("Repeat: {}", times * d);
                            roman.push(c);
                            num %= v;
                        }
                    }
                    _ => {
                        println!("Default: {}", times * d);
                        roman.push(c);
                        num %= v;
                    }
                },
            }
        } else {
            println!("Skipping..");
        }

        if num == 0 {
            break;
        }

        println!("After: {num}");
    }

    roman
}

#[test]
fn int_to_roman_test() {
    assert_eq!(int_to_roman(9), String::from("IX"));
    assert_eq!(int_to_roman(3), String::from("III"));
    assert_eq!(int_to_roman(58), String::from("LVIII"));
    assert_eq!(int_to_roman(1009), String::from("MIX"));
    assert_eq!(int_to_roman(1994), String::from("MCMXCIV"));
    assert_eq!(int_to_roman(125), String::from("CXXV"));
}
