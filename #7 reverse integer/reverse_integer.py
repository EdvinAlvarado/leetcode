def reverse(n: int) -> int:
    if n == 0:
        return 0
    ret = int(str(abs(n))[::-1]) * int(n / abs(n))
    return ret if abs(ret) < (1<<31)-1 else 0


print(reverse(-123))
print(reverse(1534236469))
print(1<<31)
