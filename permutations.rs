impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            0 => Vec::new(),
            1 => vec![nums],
            _ => {
                let mut result = Vec::new();

                for num in &nums {
                    for mut item in
                        Self::permute(nums.iter().filter(|n| *n != num).copied().collect())
                    {
                        item.push(*num);
                        result.push(item);
                    }
                }

                result
            }
        }
    }
}
