import math

def work():


    return 0

def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')

    for t0 in range(0, t):
        N,K = map(int, file.readline().split()) #destination, num horses
        cakes = []
        for i in range(N):
            cakes.append(list(map(int,file.readline().split())))

        #print(N,K,cakes)
        res = 0
        for n in range(N):
            surfaceAreas = []
            #choose different bases
            base = cakes[n]
            for i in range(len(cakes)):
                if i == n:
                    continue
                surfaceAreas.append((2*math.pi*cakes[i][0]*cakes[i][1],cakes[i][0]))

            surfaceAreas.sort()
            surfaceAreas = surfaceAreas[N-K:]
            #print(surfaceAreas)

            summ = 0

            biggestradius = 0
            for k in range(len(surfaceAreas)):

                summ+= surfaceAreas[k][0]
                if surfaceAreas[k][1]>biggestradius:
                    biggestradius = surfaceAreas[k][1]
            if biggestradius>base[0]:
                continue

            ares = summ+ math.pi*(base[0]**2)+2*math.pi*base[0]*base[1]
            if ares>res:
                res = ares
        #print (res)

        output.write("Case #{}: {:.16f}\n".format(t0 + 1,res))
    file.close()
    output.close()


main("A-large.in")