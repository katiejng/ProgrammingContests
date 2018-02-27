def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        N,K,A1,B1,C,D,E1,E2,F = map(int, file.readline().split())
        a = [A1]*(N+1)
        b = [B1]*(N+1)
        a[0],b[0]=0,0

        x = [0,A1]
        y = [0,B1]
        r = [0,0]
        s=[0,0]
        for i in range(2,N+1):

            x.append((C*x[-1]+D*y[-1]+E1)%F)
            y.append((D * x[-2] + C * y[-1] + E2) % F)
            r.append((C*r[-1] + D*s[-1]+E1)%2)
            s.append((D * r[-2] + C * s[-1] + E2) % 2)
        #print(x,y,r,s)
        for i in range(2,N+1):
            a[i]=(-1)**r[i] * x[i]
            b[i]=(-1)**s[i]*y[i]

        #print(a,b)

        c =  [[-1 for i in range(N)] for i in range(N)]

        for i in range(1,N+1):
            for j in range(1,N+1):
                c[i-1][j-1] = a[i]*b[j]
        #print(c)

        #get all total sums
        sums = []
        for aa in range(N): #starting row
            for bb in range(N): #starting column
                total = 0
                for cc in range(aa,N): #numrows
                    for dd in range(bb,N): #numcolumns

                        sums.append(sum(c,aa,bb,cc,dd))
        sums=sorted(sums)
        #print(sums)
        res =sums[-K]
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))

def sum(matrix,x,y,row,col):
    total = 0
    for i in range(x,row+1):
        for j in range(y,col+1):
            total+= matrix[i][j]
    return total

def work():
    return 0


main("B-small-attempt0.in")