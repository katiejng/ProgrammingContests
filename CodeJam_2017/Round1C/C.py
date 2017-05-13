
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    res = 0
    for t0 in range(0, t):
        N,K = map(int, file.readline().split())
        cores= []
        U =  float(file.readline().strip())
        cores=list(map(float,file.readline().split()))
        cores.sort()
        print(N,K,U,cores)
        i =0
        minValue = cores[i]
        while U>0:

            if i==N-1:
                diff = 1-cores[i]
            else:
                diff = cores[i+1]-cores[i]

            if diff*(i+1)<=U:
                minValue+=diff
                U-=diff*(i+1)
                i+=1
            else:
                #evenly split
                minValue = (minValue*(i+1)+U)/(i+1)
                U=0
            if U==0:
                break
            if U<0:
                print("ERROOR")
            if i==N:
                break

        value = minValue**(i+1)
        for x in range(i+1,N):
            value*= cores[x]
        print("Case #{}: {:.16f}\n".format(t0 + 1,value))


        output.write("Case #{}: {:.16f}\n".format(t0 + 1,value))
    file.close()
    output.close()


main("C-small-1-attempt0.in")