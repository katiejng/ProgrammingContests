import itertools
import math
imp = "IMPOSSIBLE"
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        N = int(file.readline().strip())
        #print(N)
        sticks= list(map(int, file.readline().split()))
        #print(sticks)


        #a_table = [[-1 for i in range(c)] for i in range(r)]
        res = work(sticks)
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))
def has_pair(list):
    pair = False
    prev = -1
    for stick in list:
        if prev != stick:
            prev = stick
        else:
            pair = True
            break
    if pair:
        return prev
    else:
        return -1

def do_math(a,b,c):
    try:
        if math.sqrt(math.pow(c,2)-math.pow((a-b)/2,2))>0:
            return True
        return False
    except Exception:
        return False
def is_possible_trapezium(a_list, c):

    a_list.remove(c)
    a_list.remove(c)

    a = a_list[0]
    b = a_list[1]
    if a==b:
        return False
    if do_math(a,b,c):
        return True
    else:
        return False

#ss== 0 means didn't sightsee last time
def work(sticks):
    possibilities = itertools.combinations(sticks, 4)
    results = []
    for set in possibilities:
        set=sorted(set)
        dup = has_pair(set)
        if dup!=-1:
           if is_possible_trapezium(set[:],dup):
               results.append(set)


    # get all sets of 4
    # determine if they work
        # should have 2 of the same length
        # should follow the formula of a,b,c>0
    return len(results)

main("B-small-attempt0.in")