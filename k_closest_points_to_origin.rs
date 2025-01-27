use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();

        // trying out this syntax
        points
            .into_iter()
            .for_each(|p| heap.push((-(p[0] * p[0] + p[1] * p[1]), p)));

        (0..k).map(|_| heap.pop().unwrap().1).collect()
    }
}
