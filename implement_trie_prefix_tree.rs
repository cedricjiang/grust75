use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    is_end_of_word: bool
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie{children: HashMap::new(), is_end_of_word: false}
    }
    
    fn insert(&mut self, word: String) {
        let mut curr = self;

        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(Trie::new());
        }

        curr.is_end_of_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        self.find(word).map_or(false, |v| v.is_end_of_word)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, s: String) -> Option<&Trie> {
        let mut curr = self;

        for c in s.chars() {
            if let Some(node) = curr.children.get(&c) {
                curr = node;
            } else {
                return None;
            }
        }

        Some(curr)
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
