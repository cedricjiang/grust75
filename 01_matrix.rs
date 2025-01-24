use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = mat.len();
        let cols = mat[0].len();

        // temporarily obsessed with this syntax
        let mut queue: VecDeque<(usize, usize, i32)> = (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c, 0)))
            .filter(|(r, c, _)| mat[*r][*c] == 0)
            .collect();
        let mut result: Vec<Vec<i32>> =
            mat.iter().map(|v| v.iter().map(|v| -v).collect()).collect();

        while let Some((row, col, val)) = queue.pop_front() {
            for (orow, ocol) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
                // wrapping add allows us to avoid the pain of converting
                // between signed and unsigned and check bound for only one
                // side
                let nrow = row.wrapping_add(orow);
                let ncol = col.wrapping_add(ocol);

                if nrow < rows && ncol < cols && result[nrow][ncol] == -1 {
                    result[nrow][ncol] = val + 1;
                    queue.push_back((nrow, ncol, val + 1));
                }
            }
        }

        result
    }
}
