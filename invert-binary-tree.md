# Invert a binary tree


## Python

```python 
import queue

class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root:
            return None

        q = queue.Queue()
        q.put(root)
        while not q.empty():
            node = q.get()
            node.left, node.right = node.right, node.left

            if node.left is not None:
                q.put(node.left)
            if node.right is not None:
                q.put(node.right)
        return root
```
