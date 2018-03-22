# Complete the function below.

def getDegree(elements):
    sums = [0]*((10**5)+1)
    for item in elements:
        sums[item] += 1

    return sums

def getPossibleSol(counts, index):
    if index<0 or index>=len(counts):
        return 0

    possibleResult = (counts[index]) * index
    if (index - 1 >= 0):
        possibleResult -= (counts[index - 1] * (index - 1))

    if (index + 1 < len(counts)):
        possibleResult -= (counts[index + 1] * (index + 1))

    return possibleResult

def getBestSol(dict):
    max = None
    result = None
    for item in dict:
        #print(item, dict[item])
        if max is None:
            max = dict[item]
            result = item
        elif dict[item]>max:
            max = dict[item]
            result = item

    return result



def maxPoints(elements):

    counts = getDegree(elements)
    score = 0
    possibleSols = {}
    for index in range(1,len(counts)):
        if counts[index]>0:
            possibleResult = getPossibleSol(counts, index)
            if (possibleResult!=0):
                possibleSols[index] = possibleResult
    #print(counts[:8], possibleSols)
    # be greedy and choose best option
    while counts:
        #choose greatest
        choice = getBestSol(possibleSols)
        if (not choice):
            break
        #print(choice)
        score+=counts[choice]*choice
        possibleSols.pop(choice)
        counts[choice] = 0
        if choice-1>=0:
            counts[choice-1] = 0
        if choice+1 <len(counts):
            counts[choice+1] = 0
        if not possibleSols:
            break

        for index in range(choice-1, choice+2):
            if index>0 and index < len(counts):

                possibleResult = getPossibleSol(counts, index)
                if (possibleResult != 0):
                    possibleSols[index] = possibleResult
                elif index in possibleSols:
                    possibleSols.pop(index)

        print(counts[:8], possibleSols)


    return score




elements = [3,4,2]
print(maxPoints(elements))