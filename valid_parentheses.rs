use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        // should be "static", but don't bother here
        let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

        for chr in s.chars() {
            match chr {
                '(' | '[' | '{' => stack.push(chr),
                ')' | ']' | '}' => match stack.pop() {
                    Some(ele) => {
                        if pairs.get(&ele) != Some(&chr) {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                // problem says the input string consists of parentheses only
                _ => {
                    panic!();
                }
            }
        }

        stack.is_empty()
    }
}
