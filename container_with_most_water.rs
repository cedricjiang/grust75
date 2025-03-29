use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut result = 0;

        while left < right {
            let effective_height = height[left].min(height[right]);
            let width = (right - left) as i32;

            let area = effective_height * width;

            result = result.max(area);

            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        result
    }
}
