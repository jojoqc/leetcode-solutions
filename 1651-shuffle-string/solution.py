class Solution:
    def restoreString(self, s: str, indices: List[int]) -> str:
        # sorted(indices)
        aux = {}
        aux2 = []
        for i in range(len(s)):
            aux.update({indices[i]:s[i]})
            
        for i in range(len(s)):
            aux2.append(aux[sorted(indices)[i]])
            
        return "".join(aux2)
