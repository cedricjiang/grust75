use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut result = image.clone();
        let mut queue = VecDeque::new();

        let orig_color = image[sr as usize][sc as usize];
        if orig_color != color {
            queue.push_back((sr as usize, sc as usize));
        }

        while let Some(point) = queue.pop_front() {
            result[point.0][point.1] = color;
            if point.0 > 0 && result[point.0 - 1][point.1] == orig_color {
                queue.push_back((point.0 - 1, point.1));
            }
            if point.0 < image.len() - 1 && result[point.0 + 1][point.1] == orig_color {
                queue.push_back((point.0 + 1, point.1));
            }
            if point.1 > 0 && result[point.0][point.1 - 1] == orig_color {
                queue.push_back((point.0, point.1 - 1));
            }
            if point.1 < image[0].len() - 1 && result[point.0][point.1 + 1] == orig_color {
                queue.push_back((point.0, point.1 + 1));
            }
        }

        result
    }
}
