def sum(aSet):
    r = 0
    b = 0
    for item in aSet:
        r += item[0]
        b+= item[1]
    return (r,b)

def doIt(r,b):
    num = 0
    round = 0
    results = set()
    while (r>0 or b>0) and (round<=max(r,b)):
        round +=1

        for rIndex in range(round+1):
            bIndex = round-rIndex

            rUsed,bUsed = sum(results)
            rLeft = r-rUsed
            bLeft = b-bUsed

            if (not (rIndex == 0 and bIndex == 0)) and rIndex <=rLeft and bIndex<=bLeft:
                results.add((rIndex,bIndex))
            # print(rIndex, bIndex)

            # if (rLeft < round):
            #     r = 0
            # if (bLeft < round):
            #     b = 0


    return results


def main():
    T = int(input())
    for t0 in range(T):
        r, b = map(int, input().split())
        # print("case", r, b)

        res = doIt(r,b)

        print("Case #{}: {}".format(t0 + 1, len(res)))
        # print(res)


main()