impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut result = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    Self::clear(&mut grid, i, j);
                    result += 1;
                }
            }
        }

        result
    }

    fn clear(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
        if row >= grid.len() || col >= grid[0].len() || grid[row][col] == '0' {
            return;
        }
        grid[row][col] = '0';
        for (orow, ocol) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
            // wrapping add allows us to avoid the pain of converting
            // between signed and unsigned and check bound for only one
            // side
            Self::clear(grid, row.wrapping_add(orow), col.wrapping_add(ocol));
        }
    }
}
