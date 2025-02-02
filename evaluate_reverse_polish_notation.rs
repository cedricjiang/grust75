impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            if let Ok(num) = token.parse() {
                stack.push(num);
            } else {
                // assuming valid input, it's ok to do the following
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                match token.as_str() {
                    "+" => stack.push(left + right),
                    "-" => stack.push(left - right),
                    "*" => stack.push(left * right),
                    _ => stack.push(left / right), // must be divide
                }
            }
        }

        stack.pop().unwrap()
    }
}
