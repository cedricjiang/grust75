impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 0..s.len() {
            for j in 0..=i {
                if dp[j] && word_dict.contains(&s[j..i + 1].to_string()) {
                    dp[i + 1] = true;
                    break;
                }
            }
        }

        dp[s.len()]
    }
}
