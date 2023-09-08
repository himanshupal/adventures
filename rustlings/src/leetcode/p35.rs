pub fn binary_search<T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::PartialOrd>(
    values: Vec<T>,
    target: T,
    start: usize,
    end: usize,
) -> i32 {
    if start > end {
        start as i32
    } else {
        let mid = (start + end) / 2;
        if target < values[mid] {
            binary_search(values, target, start, mid - 1)
        } else if target > values[mid] {
            binary_search(values, target, mid + 1, end)
        } else {
            mid as i32
        }
    }
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();

    if target <= nums[0] {
        0
    } else if target > nums[len - 1] {
        len as i32
    } else if target == nums[len - 1] {
        (len - 1) as i32
    } else {
        binary_search(nums, target, 0, len - 1)
    }
}

#[test]
fn search_insert_test() {
    assert_eq!(search_insert(vec![1], 1), 0);
    assert_eq!(search_insert(vec![1, 3, 5], 0), 0);
    assert_eq!(search_insert(vec![1, 3, 5], 4), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 6), 3);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 4), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
}
