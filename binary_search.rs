use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0isize;
        let mut right = (nums.len() - 1) as isize;

        while left <= right {
            let mid = left + (right - left) / 2;

            match nums[mid as usize].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid as i32,
                Ordering::Greater => right = mid - 1,
            }
        }

        -1
    }
}
