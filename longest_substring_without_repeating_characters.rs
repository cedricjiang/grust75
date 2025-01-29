use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut char_indexes = HashMap::new();
        let mut result = 0;

        for (i, c) in s.chars().enumerate() {
            char_indexes.get(&c).map(|v| left = left.max(v + 1));
            char_indexes.insert(c, i);
            result = result.max(i - left + 1);
        }

        result as i32
    }
}
