class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        aux = [i**2 for i in nums]
        return sorted(aux)
