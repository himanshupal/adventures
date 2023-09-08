pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut removed = 0;

    for (i, n) in nums.to_owned().iter().enumerate() {
        if n.eq(&val) {
            nums.remove(i - removed);
            removed += 1;
        }
    }

    nums.len() as i32
}

#[test]
fn remove_element_test() {
    assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
}
