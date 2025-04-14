impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0);

        let mut index_stack = Vec::new();
        let mut index = 0;
        let mut result = 0;

        while index < heights.len() {
            if index_stack.is_empty()
                || heights[index] > heights[index_stack[index_stack.len() - 1]]
            {
                index_stack.push(index);
                index += 1;
                continue;
            }

            let last_index = index_stack.pop().unwrap();

            let area = if index_stack.is_empty() {
                index
            } else {
                index - index_stack.last().unwrap() - 1
            } as i32
                * heights[last_index];

            result = result.max(area);
        }

        result
    }
}