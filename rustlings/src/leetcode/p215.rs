use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let capacity = k as usize;

    // preallocate memory for the capacity k+1, so it doesn't need to be reallocated while function is called
    let mut max_heap = BinaryHeap::with_capacity(capacity + 1);

    for num in nums {
        max_heap.push(Reverse(num));
        if max_heap.len() > capacity {
            max_heap.pop();
        }
    }

    // peek the top of the heap, unwrap Reversed value from Option,
    // access the actual value from Reverse via .0
    max_heap.peek().unwrap().0
}

pub fn partition(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let pivot_index = l + (r - l) / 2;
    let pivot_value = nums[pivot_index];
    nums.swap(pivot_index, r);
    let mut j = l;
    for i in l..=r {
        if nums[i] < pivot_value {
            nums.swap(i, j);
            j += 1;
        }
    }
    nums.swap(j, r);
    return j;
}

pub fn quick_select(nums: &mut Vec<i32>, l: usize, r: usize, target_index: usize) -> i32 {
    if l == r {
        return nums[l];
    }
    let i = partition(nums, l, r);
    if i == target_index {
        return nums[i];
    } else if i < target_index {
        return quick_select(nums, i + 1, r, target_index);
    } else {
        return quick_select(nums, l, i - 1, target_index);
    }
}

pub fn find_kth_largest_qs(nums: Vec<i32>, k: i32) -> i32 {
    let array = &mut nums.clone();
    return quick_select(array, 0, nums.len() - 1, nums.len() - k as usize);
}

#[test]
fn find_kth_largest_test() {
    assert_eq!(find_kth_largest_qs(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(find_kth_largest_qs(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
