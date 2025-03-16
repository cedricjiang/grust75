impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::create_subsets_recursively(vec![vec![]], &nums)
    }

    fn create_subsets_recursively(mut input: Vec<Vec<i32>>, nums: &[i32]) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return input;
        }

        let extended: Vec<Vec<i32>> = input
            .iter()
            .map(|v| {
                let mut v = v.clone();
                v.push(nums[0]);
                v
            })
            .collect();

        input.extend(extended);

        Self::create_subsets_recursively(input, &nums[1..])
    }
}
