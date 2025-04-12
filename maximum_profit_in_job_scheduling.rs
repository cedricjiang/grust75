use crate::Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32, i32)> = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((x, y), z)| (x, y, z))
            .collect();
        jobs.sort_by_key(|&(start, _, _)| start);

        let mut max_profit_from_job_pos = vec![0; jobs.len() + 1];

        for i in (0..jobs.len()).rev() {
            let (_, end, profit) = jobs[i];

            let compatible_job_pos = jobs.partition_point(|&(start, _, _)| start < end);

            max_profit_from_job_pos[i] = max_profit_from_job_pos[i + 1]
                .max(profit + max_profit_from_job_pos[compatible_job_pos]);
        }

        max_profit_from_job_pos[0]
    }
}
