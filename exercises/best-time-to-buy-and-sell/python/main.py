class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        lowest = prices[0]
        for p in prices:
            diff = p - lowest
            profit = max(profit, diff)
            lowest = min(lowest, p)
        return profit