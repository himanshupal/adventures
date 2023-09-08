pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut max = i32::MAX;
    let mut contains = false;
    for v in nums.iter().rev() {
        if v > &max {
            break;
        }
        if v == &target {
            contains = true;
            break;
        }
        max = *v;
    }
    contains
}
