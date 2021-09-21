from itertools import combinations
from typing import List

# O(n^2)
def twoSum(nums: List[int], target: int) -> List[int]:
    for j,m in enumerate(nums):
        for i,n in enumerate(nums):
            print(f'{m} {n}')
            if m + n == target and i != j: return [j,i]

# O(n(n-1)/2)
def twoSum(nums: List[int], target: int) -> List[int]:
    for j,m in enumerate(nums):
        for i,n in enumerate(nums[j:]):
            print(f'{m} {n}')
            if m + n == target and j != i+j: return [j,i+j]

# Best solution by blue_sky5
# O(n)
def twoSum(nums: List[int], target: int) -> List[int]:
    seen = {}
    for i,n in enumerate(nums):
        if target - n in seen:
            return [seen[target - n], i]
        seen[n] = i

print(twoSum([-3,4,3,90], 0))
