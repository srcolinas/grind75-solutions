# Valid Palindrome

## Python

```python
class Solution:
    def isPalindrome(self, s: str) -> bool:
        i, j = 0, len(s) - 1
        while i < j:
            ci = s[i]
            cj = s[j]

            if not ci.isalnum():
                i += 1
                continue

            if not cj.isalnum():
                j -= 1
                continue

            if ci.lower() != cj.lower():
                return False
            i += 1
            j -= 1
        return True
```

## Rust

```rust
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = chars.len() - 1;

        while i < j {
            let ci = chars[i];
            let cj = chars[j];
            if !ci.is_alphanumeric() {
                i = i + 1;
                continue
            }

            if !cj.is_alphanumeric() {
                j = j - 1;
                continue
            }

            if ci.to_ascii_lowercase() != cj.to_ascii_lowercase() {
                return false
            }

            i = i + 1;
            j = j - 1;
        }
        true
    }
}
```