impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut start = new_interval[0];
        let mut end = new_interval[1];

        let mut done_insert = false;

        for interval in intervals {
            if interval[1] < start {
                result.push(interval);
                continue;
            }

            if interval[0] > end {
                if !done_insert {
                    result.push(vec![start, end]);
                    done_insert = true;
                }
                result.push(interval);
                continue;
            }

            start = start.min(interval[0]);
            end = end.max(interval[1]);
        }

        if !done_insert {
            result.push(vec![start, end]);
        }

        result
    }
}
