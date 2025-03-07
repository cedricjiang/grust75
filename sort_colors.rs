impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut start = 0;
        let mut end = nums.len();
        let mut index = 0;

        while index < end {
            match nums[index] {
                0 => {
                    nums[index] = nums[start];
                    nums[start] = 0;
                    start += 1;
                    index += 1;
                }
                1 => {
                    index += 1;
                }
                // must be 2
                _ => {
                    nums[index] = nums[end - 1];
                    nums[end - 1] = 2;
                    end -= 1;
                }
            }
        }
    }
}
