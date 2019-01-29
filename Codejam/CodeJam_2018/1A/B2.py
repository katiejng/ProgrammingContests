def calcTimes(cashiers, division):
    worseTime = 0
    worseCashier = 0
    for cashierIndex in range(len(cashiers)):
        cashier = cashiers[cashierIndex]
        tempTime = cashier[1]*division[cashierIndex]+cashier[2]
        if tempTime>worseTime:
            worseTime = tempTime
            worseCashier = cashierIndex
    return (worseTime, worseCashier)

def getMinIndex(alist):
    index = 0
    minValue = alist[index]
    while (minValue<0):
        index+=1
        if index==len(alist):
            return -1
        minValue = alist[index]
    for i in range(len(alist)):
        if alist[i]>=0 and alist[i]<minValue:
            index = i
            minValue = alist[i]

    return index

def getSumGreaterThanZero(list):
    asum = 0
    for item in list:
        if item>0:
            asum+=1
    return asum

def getBestTime(r,b,c,cashiers, divisions):
    # if finished return
    if (b == 0):
        # print(divisions, calcTimes(cashiers, divisions))
        # print(calcTimes(cashiers, divisions)[0])
        return calcTimes(cashiers, divisions)[0]
    # else get all cashiers to give a bit to
    numCashiers = getSumGreaterThanZero(divisions)
    # print(numCashiers, "numcashiers")
    possible = []
    for dIndex in range(len(divisions)):
        # if can take the cashier
        if divisions[dIndex]>0 and cashiers[dIndex][0]>divisions[dIndex]:
            possible.append(dIndex)
        elif numCashiers<r and cashiers[dIndex][0]>divisions[dIndex]:
            possible.append(dIndex)

    # loop for each possible
    minTime = -1
    for dIndex in possible:
        tempDivisions = divisions[:]
        tempDivisions[dIndex]+=1
        time = getBestTime(r,b-1,c,cashiers, tempDivisions)
        if (time <minTime or minTime == -1):
            minTime = time
    return minTime




def main():
    T = int(input())
    for t0 in range(T):
        r, b, c = map(int, input().split())
        cashiers =[]
        for cashier in range(c):
            cashiers.append(list(map(int, input().split())))

        # print("r: {}, b: {}, c: {}".format(r,b,c))
        # print("Cashiers: {}".format(cashiers))



        result = getBestTime(r,b,c,cashiers, [0]*c)

        print ("Case #{}: {}".format(t0+1, result))


main()