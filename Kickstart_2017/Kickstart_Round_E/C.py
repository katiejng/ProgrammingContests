import itertools
import math
imp = "IMPOSSIBLE"
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        points = []
        for i in range(3):
            points.append(list(map(int, file.readline().split())))
        print(points)


        #a_table = [[-1 for i in range(c)] for i in range(r)]
        res = work(points)
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))


#ss== 0 means didn't sightsee last time
def work(points):
    minx=min(points)[0]
    maxx=max(points)[0]
    return (maxx-minx)/3/2

main("C-small-attempt0.in")