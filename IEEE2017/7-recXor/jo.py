def aux(a):
     res = [a, 1, a + 1, 0]
     return res[a % 4]

def getXORrange(a, b):
     return aux(b) ^ aux(a - 1)

t = int(input())
for _ in range(t):
    [l, h, n, d1, d2] = map(int, input().split())

    min_row = (d1 - n) // l
    min_col = (d1 - n) % l
    max_row = (d2 - n) // l
    max_col = (d2 - n) % l

    if min_row > max_row:
        min_row, max_row = max_row, min_row
    if min_col > max_col:
        min_col, max_col = max_col, min_col

    small_length = max_col - min_col + 1
    small_height = max_row - min_row + 1

    skip_number = l * min_row + min_col + n

    res = 0

    number = n
    res ^= getXORrange(number, skip_number - 1)
    number = skip_number + small_length
    for _ in range(small_height - 1):
        skip_number += l
        res ^= getXORrange(number, skip_number - 1)
        number += l
    res ^= getXORrange(number, l * h + n - 1)

    print(res)
