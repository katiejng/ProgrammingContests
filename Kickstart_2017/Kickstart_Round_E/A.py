import itertools
import math
imp = "IMPOSSIBLE"
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        string = file.readline().strip()
        res = work(string)
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))

def work(string):
    if len(string)<5:
        return len(string)
    if string[0:3]==string[3:6]:
        return 5
    if string[0:2]==string[2:4] and string[0:2]==string[4:6]:
        return 5
    return len(string)


main("A-small-attempt1.in")