def main(filename):
    file = open(filename,'r')
    t = int(file.readline())
    output = open(filename + ".out",'w')
    for t0 in range(0,t):

        #num = int(file.readline())
        #a_list = []
        
        #for j in range(names):
        #    a_list.append(file.readline().strip("\n"))
        
        #r = 5
        #c = 5
        #a_table = [[-1 for i in range(c)] for i in range(r)]
        
        res = work()
        output.write("Case #{}: {} \n".format(to+1,res))

def work():
    return 0

main("A-large.in")