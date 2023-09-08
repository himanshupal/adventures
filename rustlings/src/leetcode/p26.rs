use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> (Vec<i32>, i32) {
    let mut seen: HashSet<&i32> = HashSet::new();
    let mut removed = 0;

    let cloned = nums.clone();
    cloned.iter().enumerate().for_each(|(i, n)| {
        if seen.contains(n) {
            nums.remove(i - removed);
            removed += 1;
        } else {
            seen.insert(n);
        }
    });

    (nums.to_owned(), seen.len() as i32)

    // or do as below

    // nums.dedup();
    // (nums.to_owned(),nums.len())
}

// This also works based on the problem description; may contain unused items at end
fn remove_duplicates_another(nums: &mut Vec<i32>) -> (Vec<i32>, i32) {
    match nums.is_empty() {
        true => (vec![], 0),
        false => {
            let mut prev = 0;
            (1..nums.len()).for_each(|i| {
                if nums[prev] != nums[i] {
                    prev += 1;
                    nums[prev] = nums[i];
                }
            });
            (nums.to_owned(), (prev + 1) as i32)
        }
    }
}

#[test]
fn remove_duplicates_test() {
    assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), (vec![1, 2], 2));
    assert_eq!(remove_duplicates(&mut vec![1, 2, 2]), (vec![1, 2], 2));
    assert_eq!(
        remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
        (vec![0, 1, 2, 3, 4], 5)
    );
}
