impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut zero_count = 0;
        let mut prod = 1;

        for num in &nums {
            if *num == 0 {
                zero_count += 1;
            } else {
                prod *= num;
            }
        }

        match zero_count {
            0 => nums.iter().map(|v| prod / *v).collect(),
            1 => nums
                .iter()
                .map(|v| if *v == 0 { prod } else { 0 })
                .collect(),
            _ => vec![0; nums.len()],
        }
    }
}
