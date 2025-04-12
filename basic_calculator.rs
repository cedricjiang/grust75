impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut tokens = Vec::new();
        let mut tmp_digits = String::new();

        let mut max_level = 0;
        let mut cur_level = 0;

        let mut minus_as_neg = true;

        for c in s.chars().filter(|&x| x != ' ') {
            match c {
                '+' | '-' | '(' | ')' => {
                    if !tmp_digits.is_empty() {
                        tokens.push(tmp_digits);
                        tmp_digits = String::new();
                    }

                    if minus_as_neg && c == '-' {
                        tmp_digits = format!("{tmp_digits}{c}");
                    } else {
                        tokens.push(format!("{c}"));
                    }

                    minus_as_neg = false;

                    if c == '(' {
                        cur_level += 1;

                        if cur_level > max_level {
                            max_level = cur_level;
                        }

                        minus_as_neg = true;
                    } else if c == ')' {
                        cur_level -= 1;
                    }
                }
                _ => {
                    tmp_digits = format!("{tmp_digits}{c}");
                    minus_as_neg = false;
                }
            }
        }

        if !tmp_digits.is_empty() {
            tokens.push(tmp_digits);
        }

        let mut stack_number = vec![None; max_level + 1];
        let mut stack_operator_is_add = vec![None; max_level + 1];

        stack_number[cur_level] = Some(0);
        stack_operator_is_add[cur_level] = Some(true);

        for token in tokens {
            match token.as_str() {
                "(" => cur_level += 1,
                ")" => {
                    let num = stack_number[cur_level].unwrap();
                    stack_number[cur_level] = None;
                    cur_level -= 1;

                    if stack_operator_is_add[cur_level].unwrap_or(true) {
                        stack_number[cur_level] = Some(stack_number[cur_level].unwrap_or(0) + num);
                    } else {
                        stack_number[cur_level] = Some(stack_number[cur_level].unwrap_or(0) - num);
                    }
                    stack_operator_is_add[cur_level] = None;
                }
                "+" | "-" => stack_operator_is_add[cur_level] = Some(token == "+"),
                _ => {
                    let num: i32 = token.parse().unwrap();
                    match stack_number[cur_level] {
                        Some(n) => {
                            if stack_operator_is_add[cur_level].unwrap_or(true) {
                                stack_number[cur_level] = Some(n + num);
                            } else {
                                stack_number[cur_level] = Some(n - num);
                            }
                            stack_operator_is_add[cur_level] = None;
                        }
                        None => stack_number[cur_level] = Some(num),
                    }
                }
            }
        }

        stack_number[0].unwrap()
    }
}
