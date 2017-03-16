def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        # num = int(file.readline())
        # a_list = []
        a_string = file.readline().strip()
        b_string = file.readline().strip()

        res = work(a_string,b_string,0,0)
        # for j in range(names):
        #    a_list.append(file.readline().strip("\n"))

        # r = 5
        # c = 5

        #a_table = [[-1 for i in range(c)] for i in range(r)]
        if res:
            res = "TRUE"
        else:
            res = "FALSE"
        output.write("Case #{}: {} \n".format(t0 + 1, res))


def work(a_string, b_string, pa, pb):
    print(a_string, b_string, a_string[0:pa+1], b_string[0:pb+1],pa,pb)

    #if both a and b passed through whole string
    if pa >= len(a_string) and pb>=len(b_string):
        print(pa, pb)
        print("is it here")
        return True
    # if a or b passed through their string but the other didn't.
    if pa >= len(a_string) or pb>=len(b_string):
        print("here")
        return False
    if a_string[pa] == b_string[pb]:
        if a_string[pa]!= "*":
            print("a")
            return work(a_string,b_string,pa+1,pb+1)

    if a_string[pa] == "*":
        end = pa
        count = 0
        while end< len(a_string) and count <4:
            if a_string[end] != "*":
                count +=1
            end+=1

        for i in range(0,end+1):
            if( work(a_string, b_string, pa+1 , pb+ i)):
                return True
        return False
    elif b_string[pb] == "*":
        end = pa
        count = 0
        while end< len(a_string) and count <4:
            if a_string[end] != "*":
                count +=1
            end+=1
        for i in range(0,end+1):
            if work(a_string, b_string, pa+ i, pb+1 ):
                return True
        return False
    else:
        return False



main("B_tiny.in")