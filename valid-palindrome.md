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

_(not yet)_
