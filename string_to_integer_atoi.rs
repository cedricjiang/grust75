use std::cmp::min;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut abs = 0u64;
        let mut neg = false;
        let mut state = 0u8;

        for c in s.chars() {
            // 0: trying to strip leading whitespaces
            if state == 0 {
                if c == ' ' {
                    continue;
                }
                state = 1;
            }
            // 1: trying to locate sign
            if state == 1 {
                state = 2;
                if c == '+' {
                    continue;
                }
                if c == '-' {
                    neg = true;
                    continue;
                }
            }
            // 2: trying to parse numbers
            if state == 2 {
                match c.to_digit(10) {
                    Some(digit) => {
                        abs = abs * 10 + digit as u64;
                        if neg {
                            abs = min(abs, i32::MAX as u64 + 1);
                        } else {
                            abs = min(abs, i32::MAX as u64);
                        }
                    }
                    None => break,
                }
            }
        }

        abs as i32 * if neg { -1 } else { 1 }
    }
}
