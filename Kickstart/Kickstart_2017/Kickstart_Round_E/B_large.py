import itertools
import math
import copy
imp = "IMPOSSIBLE"
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        N = int(file.readline().strip())
        #print(N)
        sticks= list(map(int, file.readline().split()))
        #print(sticks)


        #a_table = [[-1 for i in range(c)] for i in range(r)]
        res = work(sorted(sticks))
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))

#ss== 0 means didn't sightsee last time
def work(sticks):
    #print(sticks)
    results = {}
    for i in range(len(sticks)-1):
        next = 1#only using pairs


        while sticks[i]==sticks[i+next]:
            if next ==1:  # first time
                tmpres = []
                for j in range(len(sticks)-1): # for each first element
                    if j == i or j == i+next:
                       continue
                    for k in range(j+1,len(sticks)): # for each second element
                        if k == i or k == i + next:
                            continue
                        if sticks[j]==sticks[k]:  # no rectangles allowed
                            continue
                        if sticks[k]-sticks[j]<2*sticks[i]:  # check if it is a valid trapezium
                            tmpres.append([j,k])
                        else:
                            break
                results[(i,i+next)]=tmpres
                #print(results)
            else:  # second time

                temp = copy.deepcopy(results[(i,i+next-1)])
                for j in range(len(temp)):
                    if temp[j][0]==i+next:
                        temp[j][0] = i+next-1

                    if temp[j][1] == i + next:
                        temp[j][1] = i + next - 1

                results[(i,i+next)] = temp
            next+=1
            if i+next>=len(sticks):
                break
    #print(results)

    finalres=set()
    for key,value in results.items():
        for item in value:
            finalres.add(str(sorted((key[0],key[1], item[0],item[1]))))
    #print(finalres)
    return len(finalres)

main("B-small-attempt0.in")