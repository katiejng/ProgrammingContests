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

        a_table = [[-1 for i in range(c-1)] for i in range(r-1)]
        a_table[0][0] = 1
        for C in range(c-1):
            a_table[0][C] = C+1
        for R in range(r-1):
            a_table[R][0] = R+1


        res = 0
        print (a_table)



        output.write("Case #{}: {} \n".format(t0 + 1, res))


def work():
    return 0


main("A_tiny.in")