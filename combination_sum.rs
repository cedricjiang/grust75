impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::comb_sum(&candidates, target)
    }

    fn comb_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let times = target / candidates[0];
        let mut result = Vec::new();

        if candidates.len() == 1 {
            if candidates[0] * times == target {
                return vec![vec![candidates[0]; times as usize]];
            }

            return result;
        }

        for time in 0..times + 1 {
            for mut sub_result in Self::comb_sum(&candidates[1..], target - time * candidates[0]) {
                sub_result.extend(vec![candidates[0]; time as usize]);
                result.push(sub_result);
            }
        }

        result
    }
}
