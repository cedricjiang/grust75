impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        let target = sum / 2;
        if target * 2 != sum {
            return false;
        }

        let mut reachable = vec![false; target + 1];
        reachable[0] = true;

        for num in nums {
            let num = num as usize;
            if num <= target {
                let shifted = [&vec![false; num], &reachable[..target + 1 - num]].concat();
                reachable = reachable
                    .iter()
                    .zip(shifted.iter())
                    .map(|(&x, &y)| x || y)
                    .collect();
            }
        }

        reachable[target]
    }
}
