// Fastest since it only uses math ops
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let mut reverse = 0;
        {
            let mut temp = x;
            // Scoping makes it slower for some reason
            while temp > 0 {
                reverse = 10 * reverse + (temp % 10);
                temp /= 10;
            }
        }
        x == reverse
    }
}

pub fn is_palindrome_str(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let str = x.to_string();
        let reverse: String = str.chars().rev().collect();
        str == reverse
    }
}

// Most memory efficient due to less iterations
pub fn is_palindrome_match(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let str: Vec<_> = x.to_string().chars().collect();
        let length = str.len();
        for i in 0..length / 2 {
            if str[i] != str[length - i - 1] {
                return false;
            }
        }
        true
    }
}
