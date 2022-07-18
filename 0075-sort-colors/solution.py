class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        aux0 = 0
        aux1 = 0
        aux2 = 0
        for i in range(len(nums)):
            if nums[i] == 0:
                aux0+=1
            if nums[i] == 1:
                aux1+=1
            if nums[i] == 2:
                aux2+=1
        for i in range(len(nums)):
            if i<aux0:
                nums[i] = 0
            elif i<aux0+aux1:
                nums[i] = 1
            else:
                nums[i] = 2
