import sys

triangle = makeTriangle(10)

def getPascalValue(r, c):
    return triangle[r][c]

def DoStuff(N):
    result = [[1,1]]
    sum = 1
    directions = 

    return result

def makeTriangle(x) :
    l = [1]
    result = [l]
    for i in range(x):
        # Modified v
        newlist = []
        newlist.append(l[0])
        for i in range(len(l) - 1):
            newlist.append(l[i] + l[i+1])
        newlist.append(l[-1])
        l = newlist
        result.append(l)
    return result

def main():
    T = int(input())
    
    print(triangle)
    for t0 in range(T):


        N = int(input())

        result = DoStuff(N)
        
        print("Case #{}:".format(t0 + 1))
        for row in result:
            print("{} {}".format(row[0], row[1]))


main()
