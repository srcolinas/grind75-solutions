# Two Sum

## Python

```python
class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        seen = {}
        for i, n in enumerate(nums):
            rest = target - n
            if rest in seen:
                break
            seen[n] = i
        return i, seen[rest]
```

## Rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            let rest = target - n;

            if let Some(&j) = seen.get(&rest) {
                return vec![j as i32, i as i32];
            }

            seen.insert(n, i);
        }

        vec![]
    }
}
```
