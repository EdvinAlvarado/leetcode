class Solution:
    def isPalindrome(self, x: int) -> bool:
        return int(str(x)[::-1]) == x if x >= 0 else False
