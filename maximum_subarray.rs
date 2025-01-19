impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr = i32::MIN;
        let mut result = curr;

        for num in nums {
            curr = num + curr.max(0);
            result = curr.max(result);
        }

        result
    }
}
