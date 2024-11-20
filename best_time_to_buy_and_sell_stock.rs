impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut curr_min_price = i32::MAX;

        for price in prices {
            let profit = price - curr_min_price;

            if profit < 0 {
                curr_min_price = price;
            } else if profit > result {
                result = profit;
            }
        }

        result
    }
}
