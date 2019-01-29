def calcTimes(cashiers, division):
    worseTime = 0
    for cashierIndex in range(len(cashiers)):
        cashier = cashiers[cashierIndex]
        tempTime = cashier[1]*division[cashierIndex]+cashier[2]
        if tempTime>worseTime:
            worseTime = tempTime
    return worseTime

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

def getBestTime(r,b,c,cashiers):
    division = [0]*c
    times = [0]*c
    for cashierIndex in range(c):
        times[cashierIndex] = cashiers[cashierIndex][2]
    # print(times)

    for aBit in range(b):

        # stop extra robots if too many 5 cashiers, 3 robots -> 2 0s NOOO does't work
        if division.count(0) <= (c - r):
            for divIndex in range(c):
                if division[divIndex] == 0:
                    times[divIndex] = -1

        # distribute
        # calculate which would be shortest to add a bit
        options = times[:]
        for cashierIndex in range(c):
            cashier = cashiers[cashierIndex]
            if options[cashierIndex] != -1:
                options[cashierIndex]+=cashier[1]
        chosen = getMinIndex(options)

        division[chosen]+=1
        if division[chosen]== cashiers[chosen][0]:
            # no more
            times[chosen] = -1
        else:
            times[chosen]+=cashiers[chosen][1]




    # print("Division: " , division)
    # print("Times: " , times)
    return calcTimes(cashiers, division)

def main():
    T = int(input())
    for t0 in range(T):
        r, b, c = map(int, input().split())
        cashiers =[]
        for cashier in range(c):
            cashiers.append(list(map(int, input().split())))

        # print("r: {}, b: {}, c: {}".format(r,b,c))
        # print("Cashiers: {}".format(cashiers))



        result = getBestTime(r,b,c,cashiers)

        print ("Case #{}: {}".format(t0+1, result))


main()