import math

def work():


    return 0

def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    res = 0
    for t0 in range(0, t):
        D,N = map(int, file.readline().split()) #destination, num horses
        horses = []
        for i in range(N):
            horses.append(list(map(int,file.readline().split())))

        slowestTime = 0

        for i in range(N):
            Time = (D-horses[i][0])/horses[i][1]
            #print("Time: ",Time)
            if Time>slowestTime:
                slowestTime = Time

        speed = D/slowestTime

        #print(D,N,horses)
        #print(speed)



        output.write("Case #{}: {}\n".format(t0 + 1,speed))
    file.close()
    output.close()


main("A-large.in")