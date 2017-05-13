import math


def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    res = 0
    for t0 in range(0, t):
        res = 0
        #p1 has more events!
        p1,p2 = map(int, file.readline().split())
        list1 = []
        list2 = []

        for i in range(p1):
            list1.append(list(map(int, file.readline().split())))
        for i in range(p2):
            list2.append(list(map(int, file.readline().split())))
        if len(list2)>len(list1):
            list1,list2 = list2,list1
            p1,p2 = p2,p1

        list1.sort()
        list2.sort()
        print(p1,p2,list1,list2)

        if p1 == 1:
            res= 2
        else:
            if list1[1][0] - list1[0][1] < 720 and list1[1][1] - list1[0][0] > 720:
                res= 4
            else:
                res= 2
        # if there is only one event



        print(res)
        output.write("Case #{}: {}\n".format(t0 + 1, res))

    file.close()
    output.close()


main("B.in")