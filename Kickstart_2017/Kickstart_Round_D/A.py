imp = "IMPOSSIBLE"
def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    for t0 in range(0, t):
        N, Ts, Tf = map(int, file.readline().split())
        print(N,Ts,Tf)
        businfo=[]
        for n0 in range(0,N-1):
            businfo.append(list(map(int, file.readline().split())))
        print(businfo)


        #a_table = [[-1 for i in range(c)] for i in range(r)]
        res = work(N,Ts,Tf,businfo, 0,0,0,0)
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))

#ss== 0 means didn't sightsee last time
def work(N,Ts,Tf,businfo,city,time,ss,num_sight_see):
    print("N: {},Ts: {},Tf: {},businfo: {},city: {},time: {},ss: {},num_sight_see: {}".format(N,Ts,Tf,businfo,city,time,ss,num_sight_see))

    if city==N-1:
        if time>Tf:
            return imp
        return num_sight_see

    #option 1 - sight see
    if ss ==0:
        a = work(N,Ts,Tf,businfo,city,time+Ts,1,num_sight_see+1)


    #option 2 - wait for bus
    n =0
    while businfo[city][0]+n*businfo[city][1]<time:
        n+=1
    new_time= businfo[city][0]+n*businfo[city][1]+businfo[city][2]
    b = work(N,Ts,Tf,businfo,city+1,new_time,0,num_sight_see)
    if ss==0:
        if a==imp:
            if b==imp:
                return imp
            else:
                return b
        else:
            return a
    else:
        return b

main("A-small-attempt0.in")