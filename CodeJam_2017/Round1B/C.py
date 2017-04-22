
def recurse(N, Q,horses,adjacencyMatrix,paths,time,current,currenthorse):

    #KEEP GOING UNTIL YOU ARRIVE
    while current != paths[0][1] - 1:
        # print(current)
        # CHOOSE THE NEXT SPOT
        nexti = getMaxIndex(adjacencyMatrix[current])
        # decide to switch or stay
        # print("CurrentHorse", currenthorse)

        time += adjacencyMatrix[current][nexti] / horses[currenthorse][1]
        horses[currenthorse][1] -= adjacencyMatrix[current][nexti]

        current = nexti

        nexti = getMaxIndex(adjacencyMatrix[current])
        #print (adjacencyMatrix[current],nexti)
        if adjacencyMatrix[current][nexti]==-1:
            print("HERE", paths[0][1] - 1, current)
        if horses[currenthorse][0] - adjacencyMatrix[current][nexti] < 0:
            # must swap horse
            currenthorse = nexti
        elif horses[current][0] - adjacencyMatrix[current][nexti] < 0:
            # must keep horse
            pass
        else:
            #GO BOTH PATHS
            a = recurse(N, Q,horses,adjacencyMatrix,paths,time,current,currenthorse)
            b = recurse(N, Q,horses,adjacencyMatrix,paths,time,current,current)
            print(a,b)
            return min(a,b)



    return time

def getMaxIndex(array):
    maxvalue = array[0]
    maxIndex = 0

    for i in range(1,len(array)):
        if maxvalue<array[i]:
            maxvalue = array[i]
            maxIndex = i

    return maxIndex
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    res = 0
    for t0 in range(0, t):
        N, Q = map(int, file.readline().split())
        horses= []
        for i in range(N):
            horses.append(list(map(int,file.readline().split())))
        adjacencyMatrix = []
        for i in range(N):
            adjacencyMatrix.append(list(map(int,file.readline().split())))
        paths = []
        for i in range(Q):
            paths.append(list(map(int,file.readline().split())))
        print(N, Q,horses,adjacencyMatrix,paths)

        current = paths[0][0]-1
        time = 0
        currenthorse = current

        print(recurse(N, Q,horses,adjacencyMatrix,paths,time,current,currenthorse))
        output.write("Case #{}: {}\n".format(t0 + 1,time))
    file.close()
    output.close()


main("C.in")