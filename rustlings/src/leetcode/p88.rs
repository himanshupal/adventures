/// UNSOLVED
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut temp = nums1[..m as usize].to_vec();

    temp.append(nums2);

    nums1.clear();
    temp.sort();

    temp.iter()
        .enumerate()
        .for_each(|(i, v)| nums1.insert(i, *v));

    // if m == 0 {
    //     for index in 0..n as usize {
    //         if let Some(value) = nums2.get(index) {
    //             nums1.insert(index, *value);
    //             nums1.pop();
    //         } else {
    //             break;
    //         }
    //     }
    // } else if n != 0 {
    //     let mut index = 0;
    //     let mut next_pos = 0;

    //     'outer: while let Some(value_from_2) = nums2.get(index) {
    //         'inner: loop {
    //             if let Some(value_from_1) = nums1.get(next_pos) {
    //                 println!("Value from 1: {value_from_1} | Value from 2: {value_from_2} | Index: {index} | NextPos: {next_pos}");

    //                 if value_from_1 > value_from_2 {
    //                     println!("Adding {} to nums1 at position {next_pos}", *value_from_2);
    //                     nums1.insert(next_pos, *value_from_2);
    //                     next_pos += 1;
    //                     nums1.pop();
    //                     println!("{:?}", nums1);

    //                     break 'inner;
    //                 } else if next_pos > m as usize && value_from_1 == value_from_2 {
    //                     nums1.insert(next_pos + 1, *value_from_2);
    //                     nums1.pop();
    //                     break;
    //                 } else if next_pos < m as usize {
    //                     next_pos += 1;
    //                     println!("Skipping to next.. | Index: {index} | NextPos: {next_pos} | {value_from_1}:{value_from_2}");
    //                 } else {
    //                     let remaining_values = &mut nums2[index..].to_vec();
    //                     println!("Pushing through: Index: {index} {:?}", remaining_values);

    //                     nums1.resize((m + n) as usize - remaining_values.len(), 0);
    //                     nums1.append(remaining_values);
    //                     println!("{:?}", nums1);

    //                     break 'outer;
    //                 }
    //             }

    //             // println!("{:?}", nums1);
    //         }

    //         index += 1;
    //     }
    // };
}

#[test]
fn merge_test() {
    let arr = &mut vec![1, 0];
    merge(arr, 1, &mut vec![2], 1);
    assert_eq!(*arr, vec![1, 2]);

    let arr = &mut vec![2, 0];
    merge(arr, 1, &mut vec![1], 1);
    assert_eq!(*arr, vec![1, 2]);

    let arr = &mut vec![1, 5, 7, 0, 0];
    merge(arr, 3, &mut vec![2, 6], 2);
    assert_eq!(*arr, vec![1, 2, 5, 6, 7]);

    let arr = &mut vec![4, 5, 6, 0, 0, 0];
    merge(arr, 3, &mut vec![1, 2, 3], 3);
    assert_eq!(*arr, vec![1, 2, 3, 4, 5, 6]);

    let arr = &mut vec![4, 0, 0, 0, 0, 0];
    merge(arr, 1, &mut vec![1, 2, 3, 5, 6], 5);
    assert_eq!(*arr, vec![1, 2, 3, 4, 5, 6]);

    let arr = &mut vec![1, 2, 3, 0, 0, 0];
    merge(arr, 3, &mut vec![2, 5, 6], 3);
    assert_eq!(*arr, vec![1, 2, 2, 3, 5, 6]);

    let arr = &mut vec![1];
    merge(arr, 1, &mut vec![], 0);
    assert_eq!(*arr, vec![1]);

    let arr = &mut vec![0];
    merge(arr, 0, &mut vec![1], 1);
    assert_eq!(*arr, vec![1]);

    let arr = &mut vec![1, 2, 3, 5, 15, 0, 0, 0, 0, 0];
    merge(arr, 5, &mut vec![1, 5, 10, 12, 20], 5);
    assert_eq!(*arr, vec![1, 1, 2, 3, 5, 5, 10, 12, 15, 20]);

    let arr = &mut vec![1, 2, 3, 8, 9, 0, 0, 0, 0, 0];
    merge(arr, 5, &mut vec![2, 5, 6, 8, 10], 5);
    assert_eq!(arr, &vec![1, 2, 2, 3, 5, 6, 8, 8, 9, 10]);
}
