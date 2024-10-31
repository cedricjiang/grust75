use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> =
            nums.iter().enumerate().map(|(i, num)| (*num, i)).collect();

        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(index) => {
                    if (i != *index) {
                        // the solution requires return type to be i32, not usize
                        return vec![i as i32, *index as i32];
                    }
                }
                None => {}
            }
        }

        // the problem suggests this cannot be reached
        panic!()
    }
}
