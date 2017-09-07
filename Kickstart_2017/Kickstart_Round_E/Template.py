def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        # num = int(file.readline())
        # a_list = []

        # for j in range(names):
        #    a_list.append(file.readline().strip("\n"))

        # r = 5
        # c = 5
        r,c  = map(int,file.readline().split())

        #a_table = [[-1 for i in range(c)] for i in range(r)]
        mina = min(r,c)
        minv = mina-1
        res = (minv*(minv+1)*(2*minv+1))/6
        incr = ((mina-1)*(mina))/2
        numinc = max(r,c)-min(r,c)
        res+= incr*numinc

        output.write("Case #{}: {} \n".format(t0 + 1, res))


def work():
    return 0


main("A_tiny.in")