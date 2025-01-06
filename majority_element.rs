impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                result = num;
            }
            count += if (result == num) { 1 } else { -1 };
        }

        result
    }
}
