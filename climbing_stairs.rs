impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut first = 1;
        let mut second = 1;
        let mut result = 1;

        for i in 0..n - 1 {
            result = first + second;
            first = second;
            second = result;
        }

        result
    }
}
