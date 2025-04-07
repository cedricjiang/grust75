use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if begin_word.len() != end_word.len() || !word_list.contains(&end_word) {
            return 0;
        }

        let mut word_set: HashSet<String> = word_list
            .into_iter()
            .filter(|word| word.len() == end_word.len())
            .collect();

        let mut level = 0;

        let mut queue = VecDeque::new();
        queue.push_back(begin_word);

        while !queue.is_empty() {
            level += 1;

            for _ in 0..queue.len() {
                let item = queue.pop_front().unwrap();

                if item == end_word {
                    return level;
                }

                word_set.retain(|word| {
                    if word
                        .chars()
                        .zip(item.chars())
                        .filter(|(a, b)| a != b)
                        .count()
                        == 1
                    {
                        queue.push_back(word.clone());
                        false
                    } else {
                        true
                    }
                });
            }
        }

        0
    }
}
