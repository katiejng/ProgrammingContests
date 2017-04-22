import math

def getLetter(n):
    if n==0:
        return 'R'
    elif n==1:
        return 'O'
    elif n==2:
        return 'Y'
    elif n==3:
        return 'G'
    elif n==4:
        return 'B'
    elif n==5:
        return 'V'
    else:
        return 'Z'

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
        values = list(map(int, file.readline().split()))
        N = values.pop(0)
        valuecopy = values[:]
        valuecopy.sort(reverse=True)
        #print(N,values)
        if valuecopy[0]-(valuecopy[1]+valuecopy[2])<1:
            string = [None]*N
            i = N
            maxLetter = getMaxIndex(values)
            #print(values, getMaxIndex(values))
            start = 0
            while (i>0):

                for j in range(start,N,2):

                    if values[maxLetter]!=0:
                        string[j]= getLetter(maxLetter)
                        i-=1
                        values[maxLetter]-=1
                    else:

                        maxLetter = getMaxIndex(values)
                        string[j] = getLetter(maxLetter)
                        i -= 1
                        values[maxLetter] -= 1

                if i==0:
                    break
                while not string[start] is None:
                    #print("Here",start, string)
                    start+=1

                print(values, getMaxIndex(values))

            print(string)
            output.write("Case #{}: {}\n".format(t0 + 1, ''.join(string)))
        else:
            print("FALSE")
            output.write("Case #{}: {}\n".format(t0 + 1,"IMPOSSIBLE"))
    file.close()
    output.close()


main("B-small-attempt0.in")