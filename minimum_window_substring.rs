impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut required_occurrence = [0; 128];
        for c in t.chars() {
            required_occurrence[c as usize] += 1;
        }

        let s: Vec<char> = s.chars().collect();

        let mut current_occurrence = [0; 128];

        let mut left = 0;
        let mut right = 0;
        let mut saved_left = 0;
        let mut saved_right = 0;
        let mut saved_len = s.len();

        while right < s.len() {
            if current_occurrence
                .iter()
                .zip(required_occurrence.iter())
                .all(|(x, y)| x >= y)
            {
                if right - left < saved_len {
                    saved_left = left;
                    saved_right = right;
                    saved_len = right - left;
                }
                current_occurrence[s[left] as usize] -= 1;
                left += 1;
            } else {
                current_occurrence[s[right] as usize] += 1;
                right += 1;
            }
        }

        while current_occurrence
            .iter()
            .zip(required_occurrence.iter())
            .all(|(x, y)| x >= y)
        {
            current_occurrence[s[left] as usize] -= 1;
            left += 1;
        }

        left -= 1;

        if right - left <= saved_len {
            s[left..right].iter().collect()
        } else {
            s[saved_left..saved_right].iter().collect()
        }
    }
}
