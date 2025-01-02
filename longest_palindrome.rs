use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = HashMap::new();

        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        let mut result = 0;
        let mut has_odd = false;

        for c in count.values() {
            if c % 2 == 1 {
                has_odd = true;
                result += c - 1;
            } else {
                result += c;
            }
        }

        result + if has_odd { 1 } else { 0 }
    }
}
