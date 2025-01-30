use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result = Vec::new();

        for (i, num) in nums.iter().enumerate() {
            if i > 0 && nums[i - 1] == *num {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[left] + nums[right] + num;
                if sum == 0 {
                    result.push(vec![*num, nums[left], nums[right]]);
                }
                if sum <= 0 {
                    while left < right {
                        left += 1;
                        if nums[left] != nums[left - 1] {
                            break;
                        }
                    }
                }
                if sum >= 0 {
                    while left < right {
                        right -= 1;
                        if nums[right] != nums[right + 1] {
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}
