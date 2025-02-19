use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        let mut queue = VecDeque::new();
        let mut fresh = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    queue.push_back(Some((i, j)));
                } else if grid[i][j] == 1 {
                    fresh += 1;
                }
            }
        }

        queue.push_back(None);

        let mut result = -1;

        while let Some(coor) = queue.pop_front() {
            match coor {
                Some((row, col)) => {
                    for (orow, ocol) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
                        // wrapping add allows us to avoid the pain of converting
                        // between signed and unsigned and check bound for only one
                        // side
                        let nrow = row.wrapping_add(orow);
                        let ncol = col.wrapping_add(ocol);

                        if nrow < grid.len() && ncol < grid[0].len() && grid[nrow][ncol] == 1 {
                            grid[nrow][ncol] = 2;
                            queue.push_back(Some((nrow, ncol)));
                            fresh -= 1;
                        }
                    }
                }
                None => {
                    result += 1;
                    if !queue.is_empty() {
                        queue.push_back(None);
                    }
                }
            }
        }

        if fresh == 0 {
            result
        } else {
            -1
        }
    }
}
