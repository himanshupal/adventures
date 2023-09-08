use core::num;
use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    (0..(nums.len() - k as usize + 1))
        .map(|i| {
            (i..i + k as usize).fold(i32::MIN, |p, c| if nums[c].gt(&p) { nums[c] } else { p })
        })
        .collect()
}

// UNSOLVED
pub fn max_sliding_window_alt(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut window = VecDeque::from((0..k).map(|v| nums[v as usize]).collect::<Vec<i32>>());
    let mut values = VecDeque::from([window.iter().max().unwrap().clone()]);

    (k as usize..nums.len()).for_each(|i| {
        println!("{i} -> Next: {next} Values: {:?}", values, next = nums[i]);

        let popped = window.pop_front().unwrap();

        if let Some(last) = values.get(values.len() - 1) {
            println!("Last: {last}");

            values.push_back(if nums[i] > *last || popped == *last {
                nums[i]
            } else {
                *last
            });
        }

        window.push_back(nums[i]);
    });

    values.into_iter().collect()
}

#[test]
fn max_sliding_window_test() {
    assert_eq!(
        max_sliding_window_alt(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(max_sliding_window_alt(vec![1], 1), vec![1]);
    assert_eq!(max_sliding_window_alt(vec![1, -1], 1), vec![1, -1]);
    assert_eq!(max_sliding_window_alt(vec![1, 0], 2), vec![1]);
    assert_eq!(max_sliding_window_alt(vec![5, 3, 4], 1), vec![5, 3, 4]);
    assert_eq!(max_sliding_window_alt(vec![5, 3, 4], 2), vec![5, 4]);
}
