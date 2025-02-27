impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();

        let mut result = Vec::new();

        for interval in intervals {
            if result
                .last()
                .map_or(true, |v: &Vec<i32>| v[1] < interval[0])
            {
                result.push(interval);
            } else {
                result.last_mut().unwrap()[1] = interval[1];
            }
        }

        result
    }
}
