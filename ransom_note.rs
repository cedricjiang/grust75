impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = [0; 26];

        for c in magazine.chars() {
            let index = c as usize - 'a' as usize;
            count[index] += 1;
        }

        for c in ransom_note.chars() {
            let index = c as usize - 'a' as usize;
            if count[index] == 0 {
                return false;
            }
            count[index] -= 1;
        }

        true
    }
}
