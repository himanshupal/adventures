pub fn to_decimal(mut s: String) -> u128 {
    let mut value = 0;
    let mut i = 0;

    while i != s.len() {
        let num = s.chars().nth_back(i).unwrap().to_digit(10).unwrap() as u128;
        value += num * 2u128.pow(i as u32);
        i += 1;
    }

    value
}

pub fn to_binary(mut n: u128) -> String {
    let mut value = String::new();

    if n == 0 {
        return "0".to_string();
    }

    while n != 0 {
        value = format!("{:b}{value}", n % 2);
        n /= 2;
    }

    value
}

// Not working for longer strings
pub fn add_binary(a: String, b: String) -> String {
    to_binary(to_decimal(a) + to_decimal(b))
}

// One liner similar to `add_binary_other` below; uses state instead of creating new vars
pub fn add_binary_easy(a: String, b: String) -> String {
    (0..)
        .scan(
            (0, a.chars().rev(), b.chars().rev()),
            |(carry, a, b), _| match (*carry, a.next(), b.next()) {
                (0, None, None) => None, // break when no carry and no more digits
                (c, a, b) => {
                    let a = a.map_or(0, |v| v.to_digit(2).unwrap_or(0));
                    let b = b.map_or(0, |v| v.to_digit(2).unwrap_or(0));
                    let sum = a + b + c;
                    *carry = sum >> 1;
                    char::from_digit(sum & 1, 2)
                }
            },
        )
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

// Involves complex arithmetics
pub fn add_binary_mask(a: String, b: String) -> String {
    // Swap pointers so a is largest
    let mut a = &a;
    let mut b = &b;
    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }

    // unsafe from_utf8_unchecked is compiled to no-op,
    // otherwise the string is rechecked as valid utf8
    let mut res = a
        .as_bytes()
        .rchunks(127)
        .zip(
            b.as_bytes()
                .rchunks(127)
                .chain(std::iter::repeat(&[b'0'; 127][..])),
        )
        .fold((String::new(), 0), |s, (a, b)| {
            let sum = unsafe {
                s.1 + u128::from_str_radix(std::str::from_utf8_unchecked(a), 2).unwrap_or(0)
                    + u128::from_str_radix(std::str::from_utf8_unchecked(b), 2).unwrap_or(0)
            };
            (
                format!("{:0127b}", sum & 0x7fffffffffffffffffffffffffffffff) + &s.0,
                sum >> 127,
            )
        });

    // final carry and special cases
    if (res.1 == 1) {
        "1".to_string() + &res.0
    } else {
        let str = res.0.trim_start_matches("0").to_string();
        if str.len() > 0 {
            str
        } else {
            "0".to_string()
        }
    }
}

// Easy to understand & works for any length
pub fn add_binary_other(a: String, b: String) -> String {
    const ZERO: u32 = '0' as u32;

    let mut carry = 0;
    let mut a = a.chars().rev();
    let mut b = b.chars().rev();
    let mut ans = String::new();

    loop {
        match (a.next(), b.next()) {
            (None, None) => {
                if carry == 1 {
                    ans = format!("{:b}{ans}", 1);
                }
                break;
            }
            (a_char, b_char) => {
                let a = a_char.unwrap_or('0') as u32;
                let b = b_char.unwrap_or('0') as u32;
                let sum = a + b + carry - ZERO * 2;
                ans = format!("{:b}{ans}", sum % 2);
                carry = sum / 2;
            }
        }
    }

    ans
}

#[test]
fn add_binary_test() {
    assert_eq!(
        add_binary_other("0".to_string(), "0".to_string()),
        "0".to_string()
    );
    assert_eq!(
        add_binary_other("11".to_string(), "1".to_string()),
        "100".to_string()
    );
    assert_eq!(
        add_binary_other("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
    assert_eq!(
      add_binary_other("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(), "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()),
        "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string()
    );
    assert_eq!(
      add_binary_easy("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()),
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
    assert_eq!(
      add_binary_other("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()),
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
    assert_eq!(
      add_binary_mask("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()),
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
}
