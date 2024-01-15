def div_and_remainder(n, d):
    if d == 0:
        raise Exception("0での除算はできません")
    return n / d, n % d

v = div_and_remainder(5, 2)
print(v)
