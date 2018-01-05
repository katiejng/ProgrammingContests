import math

def work(N,P,req_weights,actual_weights):
    res = 0
    while True:# FOR EACH PACKET
        for i in range(N):
            if len(actual_weights[i])==0:
                return res
        print("boop")

        current = actual_weights[0][0]
        # find how much to multiply it

        maxi = (10*actual_weights[0][0]//(9*req_weights[0]))
        min = math.ceil(10*actual_weights[0][0]/(11*req_weights[0]))
        #pass through and see if it worked
        flag = False
        for multi in range(min, maxi+1):
            flag = True
            for j in range(N): # each j is an ingredient
                amin = multi* req_weights[j] * 9 // 10
                amax = (multi* req_weights[j] * 11 +10-1) // 10
                print("looking for", amin, amax)
                if not(amin <=actual_weights[j][0]<=amax):
                    print("this failed")
                    flag = False

            if flag is True:
                print("this passed")
                break


        if flag is True:
            print("did this happen?")
            res+=1
            for i in range(N):
                actual_weights[i].pop(0) # POP off everything
        else:
            #pop the smallest off
            minIndex = 0
            minValue = actual_weights[0][0]/req_weights[0]
            for i in range(1,N):
                if actual_weights[i][0]/req_weights[i]<minValue:
                    minIndex = i
                    minValue = actual_weights[i][0]/req_weights[i]
            actual_weights[minIndex].pop(0)



    return res

def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        N, P = map(int, file.readline().split())
        req_weights = list(map(int, file.readline().split()))
        actual_weights= []
        for i in range(N):
            actual_weights.append(list(sorted(map(int,file.readline().split()))))

        print(N, P, req_weights, actual_weights)

        res = work(N,P,req_weights,actual_weights)
        print(N, P, req_weights, actual_weights)

        print(t0+1, "res: ", res)
        print("next")
        #print(actual_weights)

        output.write("Case #{}: {}\n".format(t0 + 1,res))
    file.close()
    output.close()


main("B-large-practice.in")