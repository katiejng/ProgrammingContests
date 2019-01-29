def TripleSort(a_list):
    done = False
    while not done:
        done = True
        for i in range (len(a_list)-2):
            if a_list[i]>a_list[i+2]:
                done = False
                a_list[i+2], a_list[i] = a_list[i], a_list[i+2]

    return (a_list)


def main():
    T = int(input())
    for t0 in range(T):
        n = int(input())
        numbers = list(map(int, input().split()))
        tripleSorted = TripleSort(numbers[:])
        realSorted = sorted(numbers)
        resultIndex = -1
        for index in range(len(realSorted)):
            if realSorted[index] != tripleSorted[index]:
                resultIndex = index
                break

        if resultIndex == -1:
            resultIndex = "OK"



        print ("Case #{}: {}".format(t0+1, resultIndex))

main()