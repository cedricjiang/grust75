impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return Vec::new();
        }

        let s: Vec<char> = s.chars().collect();

        let mut end_indices = Vec::new();

        let mut p_count = [0; 26];
        let mut s_count = [0; 26];

        for c in p.chars() {
            p_count[c as usize - 'a' as usize] += 1;
        }
        for i in 0..p.len() {
            s_count[s[i] as usize - 'a' as usize] += 1;
        }
        if p_count == s_count {
            end_indices.push(p.len());
        }

        for i in p.len()..s.len() {
            s_count[s[i] as usize - 'a' as usize] += 1;
            s_count[s[i - p.len()] as usize - 'a' as usize] -= 1;

            if p_count == s_count {
                end_indices.push(i + 1);
            }
        }

        end_indices
            .into_iter()
            .map(|x| (x - p.len()) as i32)
            .collect()
    }
}
