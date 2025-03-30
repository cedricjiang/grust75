use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: HashSet<i32> = (0..n).collect();

        let mut connections = vec![HashSet::new(); n as usize];

        for edge in edges.iter() {
            connections[edge[0] as usize].insert(edge[1]);
            connections[edge[1] as usize].insert(edge[0]);
        }

        while result.len() > 2 {
            let mut to_remove = Vec::new();

            for i in 0..n {
                if connections[i as usize].len() == 1 {
                    result.remove(&i);
                    to_remove.push((connections[i as usize].drain().next().unwrap(), i));
                }
            }

            for (from, to) in to_remove {
                connections[from as usize].remove(&to);
            }
        }

        result.into_iter().collect()
    }
}
