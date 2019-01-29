

def work(S, alist):
    N = []
    M = []
    start = 0
    length = 0
    count = 0
    # for each sign
    for i in range(S):
        M.append(alist[i][0]+alist[i][1])
        N.append(alist[i][0]-alist[i][2])
    nValues = set(N)
    mValues = set(M)
    # print(N,M)
    # print(nValues, mValues)

    maxSequence = 0
    sequenceCount = 1
    startIndexes = []
    for startIndex in range(S-1):
        for n in nValues:
            for m in mValues:
                curSequence = 0
                for index in range(startIndex, S):

                    if (N[index] == n or M[index] == m):
                        curSequence += 1
                    else:
                        break
                if (curSequence > maxSequence):
                    # replace it
                    maxSequence = curSequence
                    startIndexes = [startIndex]

                    sequenceCount = 1

                elif (curSequence == maxSequence):
                    # print("BEST", startIndexes, startIndex)
                    if not (startIndex in startIndexes):
                        # add to it
                        sequenceCount += 1
                        startIndexes.append(startIndex)
            # print(curSequence, startIndex, maxSequence, sequenceCount)


    # for n in nValues:
    #     for m in mValues:
    #         for startIndex in range(S-1):
    #             curSequence = 0
    #             for index in range(startIndex, S):
    #                 if (N[index] == n or M[index] == m):
    #                     curSequence+=1
    #                 else:
    #                     break
    #             if (curSequence>maxSequence):
    #                 # replace it
    #                 maxSequence = curSequence
    #                 sequenceCount = 1
    #             elif (curSequence == maxSequence):
    #                 # add to it
    #                 sequenceCount+=1
    #
    #             print(curSequence, sequenceCount)


    return "{} {}".format(maxSequence, sequenceCount)




def main():
    T = int(input())
    for t0 in range(T):
        S = int(input())
        alist = []
        for i in range(S):
            alist.append(list(map(int, input().split())))

        # print(S, alist)
        if (S == 1):
            result = "1 1"
        else:
            result = work(S, alist)



        print ("Case #{}: {}".format(t0+1, result))

main()