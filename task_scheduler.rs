use std::collections::HashMap;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut occurrences = HashMap::new();

        for task in &tasks {
            *occurrences.entry(task).or_insert(0) += 1;
        }

        let mut occurrences: Vec<_> = occurrences.into_values().collect();

        occurrences.sort_unstable();

        let max_occur = occurrences.pop().unwrap() - 1;

        let mut idle_count = max_occur * n;

        while let Some(occur) = occurrences.pop() {
            idle_count -= occur.min(max_occur);
            if idle_count < 0 {
                return tasks.len() as i32;
            }
        }

        tasks.len() as i32 + idle_count
    }
}
