use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;
        let mut lowest = prices[0];
        for p in prices.iter() {
            profit = cmp::max(profit, p - lowest);
            lowest = cmp::min(lowest, *p);
        }
        profit
    }
}