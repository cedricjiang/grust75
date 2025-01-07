impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.chars().rev().map(|x| x as u8 - '0' as u8);
        let mut b = b.chars().rev().map(|x| x as u8 - '0' as u8);

        let mut result = Vec::new();
        let mut carry = 0;

        while let Some(digit_a) = a.next() {
            // if b is shorter, digit_b will just be 0 after b is consumed
            let digit_b = b.next().unwrap_or(0);
            let sum = digit_a + digit_b + carry;
            result.push(sum % 2);
            carry = sum / 2;
        }

        // if b is longer, we will enter this loop and process the remainder
        while let Some(digit_b) = b.next() {
            let sum = digit_b + carry;
            result.push(sum % 2);
            carry = sum / 2;
        }

        if carry == 1 {
            result.push(1);
        }

        result
            .into_iter()
            .rev()
            .map(|x| (x + '0' as u8) as char)
            .collect()
    }
}
