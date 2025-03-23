impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as i64 - 1;
        let mut n = n as i64 - 1;

        let m = m + n;
        if (n * 2 > m) {
            n = m - n;
        }

        let mut result = 1;
        for i in 0..n {
            result *= (m - i);
            result /= (i + 1);
        }
        result as i32
    }
}
