
import math

def calcSum(N, res):
    result = 0
    for item in res:
        result += round(item/N*100,0)
    return result


def calcPerc(n, res, numberNeeded):
    percentage = 0
    for language in res:
        percentage += language * 100 // n
        if numberNeeded[language] == 0:
            percentage += 1
    return percentage

def work (N, L, res):
    numberNeeded = {}
    numberNeeded[N] = 0
    for i in range (N-1,-1,-1):
        # check if rounds up
        if (round(i/N,2) >i/N):
            numberNeeded[i] = 0
        else:
            numberNeeded[i] = numberNeeded[i+1]+1


    res.sort(key=lambda k: math.inf if numberNeeded[k] == 0 else
    numberNeeded[k])

    # print(res)

    remainder = N-sum(res)
    index = 0
    while remainder>=0:
        if (res[index] == 0):
            res.append(remainder)
            break
        numToAdd = numberNeeded[res[index]]
        if numToAdd-res[index]<=remainder:
            res[index]=numToAdd
            remainder = N - sum(res)
        index+=1


    return calcPerc(N, res, numberNeeded)






def main():
    T = int(input())
    for t0 in range(T):
        N, L = map(int,input().split())
        res = list(map(int, input().split()))


        res.append(0)
        # print(N, L, res)
        result = work (N, L, res)


        print ("Case #{}: {}".format(t0+1, result))

main()