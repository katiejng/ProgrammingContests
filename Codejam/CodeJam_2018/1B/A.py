def getNumberNeeded(N):
    numberNeeded = dict()

    numberNeeded[N] = math.inf
    for j in range(N - 1, -1, -1):
        percentage = (j / N) * 100
        roundingNumber = int((percentage % 1) * 10)
        if roundingNumber >= 5:
            numberNeeded[j] = 0
        else:
            numberNeeded[j] = numberNeeded[j + 1] + 1
    return numberNeeded

import math

t = int(input())

for i in range(1, t + 1):
    N, L = map(int, input().split())
    res = list(map(int, input().split()))
    res.append(0)
    remainingPeople = N - sum(res)

    numberNeeded = getNumberNeeded(N)

    res.sort(key=lambda k: math.inf if numberNeeded[k] == 0 else numberNeeded[k])

    for j in range(len(res)):
        peopleNeeded = numberNeeded[res[j]]
        if res[j] == 0:
            while peopleNeeded <= remainingPeople:
                res.append(peopleNeeded)
                remainingPeople -= peopleNeeded
            break
        elif peopleNeeded <= remainingPeople:
            res[j] += peopleNeeded
            remainingPeople -= peopleNeeded
        else:
            break

    res.append(remainingPeople)


    percentage = 0
    for language in res:
        percentage += language * 100 // N
        if numberNeeded[language] == 0:
            percentage += 1

    print("Case #{}: {}".format(i, percentage))
