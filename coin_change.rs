impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut result = vec![i32::MAX; amount + 1];
        result[0] = 0;

        for i in 1..amount + 1 {
            for coin in &coins {
                let coin = *coin as usize;
                if i >= coin && result[i - coin] != i32::MAX {
                    result[i] = result[i].min(result[i - coin] + 1)
                }
            }
        }

        if result[amount] == i32::MAX {
            -1
        } else {
            result[amount]
        }
    }
}
