use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_highest = Vec::with_capacity(height.len());
        let mut curr = i32::MIN;
        let mut result = 0;

        // after this loop, highest will contain for each point the highest
        // elevation seen from its left (self included)
        for h in &height {
            curr = cmp::max(*h, curr);
            left_highest.push(curr);
        }

        curr = i32::MIN;

        // reverse iterate to get for each point the highest elevation seen
        // from its right (self included), the smaller of this and above will
        // be the "wall" height when considering water accumulation for this
        // point
        for i in (0..height.len()).rev() {
            curr = cmp::max(height[i], curr);
            let wall = cmp::min(left_highest[i], curr);
            result += wall - height[i];
        }

        result
    }
}
