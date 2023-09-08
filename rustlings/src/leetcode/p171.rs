pub fn title_to_number(column_title: String) -> i32 {
    let mut column_index = 0;

    for index in 0..column_title.len() {
        if let Some(c) = column_title.chars().nth_back(index) {
            println!("Char: {c}, Pow: {}", 26u32.pow(index as u32));
            column_index += 26u32.pow(index as u32) * ((c as u32) - 64);
        }
    }

    column_index.try_into().unwrap()
}

#[test]
fn title_to_number_test() {
    assert_eq!(title_to_number(String::from("A")), 1);
    assert_eq!(title_to_number(String::from("AB")), 28);
    assert_eq!(title_to_number(String::from("ZY")), 701);
    assert_eq!(title_to_number(String::from("AZ")), 52);
    assert_eq!(title_to_number(String::from("ZY")), 701);
    assert_eq!(title_to_number(String::from("FXSHRXW")), 2147483647);
}
