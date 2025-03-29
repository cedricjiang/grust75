impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec!["".to_string()];

        for digit in digits.chars() {
            result = Self::add_letters(
                result,
                match digit {
                    '2' => vec!['a', 'b', 'c'],
                    '3' => vec!['d', 'e', 'f'],
                    '4' => vec!['g', 'h', 'i'],
                    '5' => vec!['j', 'k', 'l'],
                    '6' => vec!['m', 'n', 'o'],
                    '7' => vec!['p', 'q', 'r', 's'],
                    '8' => vec!['t', 'u', 'v'],
                    _ => vec!['w', 'x', 'y', 'z'], // must be 9
                },
            );
        }

        result
    }

    fn add_letters(input: Vec<String>, letters: Vec<char>) -> Vec<String> {
        let mut output = Vec::new();

        for item in input {
            for letter in &letters {
                output.push(format!("{item}{letter}"));
            }
        }

        output
    }
}
