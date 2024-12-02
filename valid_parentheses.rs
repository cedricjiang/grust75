use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        // should be "static", but don't bother here
        let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

        for chr in s.chars() {
            match chr {
                '(' | '[' | '{' => stack.push(chr),
                // problem says the input string consists of parentheses only
                // so this must match ending bracket types
                _ => {
                    if let Some(ele) = stack.pop() {
                        if pairs.get(&ele) != Some(&chr) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}
