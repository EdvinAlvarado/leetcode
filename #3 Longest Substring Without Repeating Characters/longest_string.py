def lengthOfLongestSubstring(s: str) -> int:
    top = 0
    for i,c in enumerate(s):
        unique = c
        temp =1
        if i + 1 == len(s):
            top = max(top, temp)
            break
        for j in range(i+1, len(s)):
            if s[j] not in unique:
                unique += s[j]
                temp += 1
            else:
                break
        top = max(top, temp)
    print(top)
    return top

print(lengthOfLongestSubstring(" "))
