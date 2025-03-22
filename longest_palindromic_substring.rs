impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut result = s[0..1].to_string();

        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        for i in 0..len {
            let max_steps = i.min(len - i - 1);
            let mut viable_steps = 0;
            while viable_steps < max_steps {
                if chars[i + viable_steps + 1] != chars[i - viable_steps - 1] {
                    break;
                }
                viable_steps += 1;
            }
            if viable_steps * 2 + 1 > result.len() {
                result = s[i - viable_steps..=i + viable_steps].to_string();
            }
        }

        for i in 0..len - 1 {
            if chars[i] != chars[i + 1] {
                continue;
            }
            let max_steps = i.min(len - i - 2);
            let mut viable_steps = 0;
            while viable_steps < max_steps {
                if chars[i + viable_steps + 2] != chars[i - viable_steps - 1] {
                    break;
                }
                viable_steps += 1;
            }
            if viable_steps * 2 + 2 > result.len() {
                result = s[i - viable_steps..=i + 1 + viable_steps].to_string();
            }
        }

        result
    }
}
