use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, num)| (*num, i)).collect();

        for (i, num) in nums.iter().enumerate() {
            if let Some(&index) = map.get(&(target - num)) {
                if (i != index) {
                    // the solution requires return type to be i32, not usize
                    return vec![i as i32, index as i32];
                }
            }
        }

        panic!()
    }
}
