use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut prereq_counts = vec![0; num_courses as usize];
        let mut dep_map = HashMap::new();

        for pair in prerequisites {
            prereq_counts[pair[0] as usize] += 1;
            dep_map
                .entry(pair[1] as usize)
                .or_insert(Vec::new())
                .push(pair[0] as usize);
        }

        let mut queue = VecDeque::new();
        for (i, pc) in prereq_counts.iter().enumerate() {
            if *pc == 0 {
                queue.push_back(i);
            }
        }

        while let Some(course) = queue.pop_front() {
            if let Some(deps) = dep_map.get(&course) {
                for dep in deps {
                    prereq_counts[*dep] -= 1;
                    if prereq_counts[*dep] == 0 {
                        queue.push_back(*dep);
                    }
                }
            }
        }

        prereq_counts.iter().all(|x| *x == 0)
    }
}
