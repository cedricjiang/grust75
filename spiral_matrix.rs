impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut round = 0;

        let height = matrix.len();
        let width = matrix[0].len();

        while width > round * 2 && height > round * 2 {
            let this_width = width - round * 2;
            let this_height = height - round * 2;

            let this_start = (round, round);

            if this_height == 1 {
                for i in 0..this_width {
                    result.push(matrix[this_start.0][this_start.1 + i]);
                }
            } else if this_width == 1 {
                for i in 0..this_height {
                    result.push(matrix[this_start.0 + i][this_start.1]);
                }
            } else {
                for i in 0..this_width - 1 {
                    result.push(matrix[this_start.0][this_start.1 + i]);
                }
                for i in 0..this_height - 1 {
                    result.push(matrix[this_start.0 + i][this_start.1 + this_width - 1]);
                }
                for i in 0..this_width - 1 {
                    result.push(
                        matrix[this_start.0 + this_height - 1][this_start.1 + this_width - 1 - i],
                    );
                }
                for i in 0..this_height - 1 {
                    result.push(matrix[this_start.0 + this_height - 1 - i][this_start.1]);
                }
            }

            round += 1;
        }

        result
    }
}
