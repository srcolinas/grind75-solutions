# Best Time to Buy and Sell Stock

## Python

```python
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        lowest = prices[0]
        for p in prices:
            diff = p - lowest
            profit = max(profit, diff)
            lowest = min(lowest, p)
        return profit
```

## Rust

```rust
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
```
