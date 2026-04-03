class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        seen = {}
        for i, n in enumerate(nums):
            rest = target - n
            if rest in seen:
                break
            seen[n] = i
        return i, seen[rest]