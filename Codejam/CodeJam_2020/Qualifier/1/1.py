import sys


def getTrace(rows):
    sum = 0
    for i in range(len(rows)):
        sum += rows[i][i]
    return sum


def getRows(rows):
    sum = 0
    for r0 in range(len(rows)):
        tracker = {}
        col = rows[r0]
        for c0 in range(len(col)):
            val = col[c0]
            if val in tracker:
                sum += 1
                break
            else:
                tracker[val] = True

    return sum


def getCols(rows):
    sum = 0
    for r0 in range(len(rows)):
        tracker = {}

        for c0 in range(len(rows)):
            val = rows[c0][r0]
            if val in tracker:
                sum += 1
                break
            else:
                tracker[val] = True

    return sum


def main():
    T = int(input())
    for t0 in range(T):
        n = int(input())
        rows = []
        for n0 in range(n):
            rows.append(list(map(int, input().split())))
        trace = getTrace(rows)
        rowCount = getRows(rows)
        colCount = getCols(rows)
        print("Case #{}: {} {} {}".format(t0 + 1, trace, rowCount, colCount))


main()
