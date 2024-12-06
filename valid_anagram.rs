impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut count_arr = [0; 26];

        for c in s.chars() {
            count_arr[c as usize - 'a' as usize] += 1;
        }

        for c in t.chars() {
            count_arr[c as usize - 'a' as usize] -= 1;
        }

        count_arr.iter().all(|&v| v == 0)
    }
}
