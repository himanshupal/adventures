fn get_chars(list: &mut Vec<u32>, num: u32) {
    let rem_check = |n: u32| {
        if n % 26 == 0 {
            (26, n / 26 - 1)
        } else {
            (n % 26, n / 26)
        }
    };

    if let Some(value) = list.get(0) {
        if num > 26 {
            let (rem, div) = rem_check(num);
            list.insert(0, rem);
            get_chars(list, div);
        } else {
            list.insert(0, num);
        }
    } else {
        let (rem, div) = rem_check(num);
        list.push(rem);
        get_chars(list, div);
    }
}

pub fn convert_to_title(column_number: i32) -> String {
    let mut unsigned = column_number as u32;
    if column_number > 26 {
        let char_list = &mut vec![];
        let mut title = String::new();
        get_chars(char_list, unsigned);
        char_list.iter().for_each(|i| {
            if let Some(ch) = char::from_u32(i + 64) {
                title.push(ch);
            }
        });
        title
    } else {
        char::from_u32(unsigned + 64).unwrap().to_string()
    }
}

#[test]
fn convert_to_title_test() {
    assert_eq!(convert_to_title(1), String::from("A"));
    assert_eq!(convert_to_title(28), String::from("AB"));
    assert_eq!(convert_to_title(52), String::from("AZ"));
    assert_eq!(convert_to_title(701), String::from("ZY"));
    assert_eq!(convert_to_title(2147483647), String::from("FXSHRXW"));
}
