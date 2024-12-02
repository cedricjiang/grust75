impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // rust strings are not indexable to chars so this is to simulate other
        // languages; in reality, if we do this, we might as well filter out
        // non-alphanumeric and convert the case, so the check will be trivial
        let s: Vec<char> = s.chars().collect();

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            while left < right && !s[left].is_alphanumeric() {
                left += 1;
            }

            while left < right && !s[right].is_alphanumeric() {
                right -= 1;
            }

            if left == right {
                break;
            }

            if s[left].to_ascii_lowercase() != s[right].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}
