def sumSquare(amap, r1, r2, c1, c2):

    asum = 0
    # print(amap[r1:r2])
    for row in amap[r1:r2]:
        asum+=row[c1:c2].count('@')
        # print(row[c1:c2])

    return asum


def allEqual(alist, num):
    for value in alist:
        if value != num:
            return False

    return True

def sumCol(alist, col):
    asum = 0
    for row in alist:
        if row[col] == "@":
            asum+=1

    return asum


def greaterEqual(alist, num):
    for value in alist:
        if value > num:
            print(value, num)
            return False

    return True

def isPossible():
    r, c, h, v = map(int, input().split())
    amap = []
    numChips = 0
    for i in range(r):
        amap.append(input())
        numChips += amap[i].count("@")

    people = (h + 1) * (v + 1)

    # check if divisible
    numPerPerson = numChips // people
    if not (people * numPerPerson == numChips):
        return False
    if (numChips == 0):
        return True
    lastCut = 0
    hcuts = [0]
    rowChips = 0
    for i in range(lastCut, r):
        rowChips += amap[i].count("@")
        if (rowChips == numPerPerson * (v + 1)):
            # got enough
            hcuts.append(i + 1)
            lastCut = i + 1
            rowChips = 0
        if (rowChips > numPerPerson * (v + 1)):
            return False

    # # For each vertical cut
    # numChips = [0]*(h+1)
    # c1 = 0
    # for i in range(v):  #for each vertical cut
    #     numPerSpot = [0]*(h+1)
    #
    #     for c2 in range(c1, c): # calculate how wide it needs to be
    #         for cube in range(len(hcuts)-1):
    #             numPerSpot[cube] = sumSquare(amap, c1, c2, hcuts[cube], hcuts[cube+1])
    #
    #         # if enough break
    #         if (allEqual(numPerSpot, numPerPerson)):
    #             c1 = c2+1
    #             numChips = [0] * (h + 1)
    #             continue
    #         # if not enough loop
    #     print(numChips, numPerPerson)
    #     if allEqual(numChips, numPerPerson):  # if it adds up, reset the chips, and lastcut
    #         # reset
    #         numChips = [0] * (h + 1)
    #
    #     else:
    #
    #         return False
    #
    vcuts = [0]

    lastCut = 0
    vcuts = [0]
    colChips = 0
    for i in range(lastCut, c):
        colChips += sumCol(amap, i)
        if (colChips == numPerPerson * (h + 1)):
            # got enough
            vcuts.append(i + 1)
            lastCut = i + 1
            colChips = 0
        if (colChips > numPerPerson * (h + 1)):
            return False

    ## add it
    for hIndex in range(len(hcuts)-1):
        for vIndex in range(len(vcuts)-1):
            # print(hcuts[hIndex], hcuts[hIndex+1], vcuts[vIndex], vcuts[vIndex+1])
            if (sumSquare(amap, hcuts[hIndex], hcuts[hIndex + 1], vcuts[vIndex], vcuts[vIndex + 1])!=numPerPerson):
                return False



    # print("hcuts: {} vcuts: {}".format(hcuts, vcuts))
    # print("r: {} c: {} h: {} v:{}".format(r, c, h, v))
    # print("people: {} numchips: {} numperperson: {}".format(people, numChips, numPerPerson))
    # print(amap)
    return True

def main():
    T = int(input())
    for t0 in range(T):

        result = isPossible()
        if (result):
            result = "POSSIBLE"
        else:
            result = "IMPOSSIBLE"
        print ("Case #{}: {}".format(t0+1, result))

main()