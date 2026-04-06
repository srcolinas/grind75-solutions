# Valid Parentheses

## Python

```python
class Solution:
    def isValid(self, s: str) -> bool:
        stack = []

        for c in s:
            match c:
                case "(" | "[" | "{":
                    stack.append(c)
                case ")":
                    if not stack or stack.pop() != "(":
                        return False
                case "]":
                    if not stack or stack.pop() != "[":
                        return False
                case "}":
                    if not stack or stack.pop() != "{":
                        return False
                case _:
                    # Optional: handle unexpected characters
                    return False

        return not stack
```

## Rust

```rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                // If it's an opening bracket, just push it
                '(' | '{' | '[' => stack.push(c),

                // If it's a closing bracket, pop and check
                ')' => if stack.pop() != Some('(') { return false; },
                '}' => if stack.pop() != Some('{') { return false; },
                ']' => if stack.pop() != Some('[') { return false; },

                // Handle any other unexpected characters (optional)
                _ => return false,
            }
        }

        // Must be empty to be valid
        stack.is_empty()
    }
}
```
