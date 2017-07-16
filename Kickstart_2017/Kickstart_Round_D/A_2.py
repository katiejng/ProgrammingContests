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


        a_table = [[-1 for i in range(N)] for i in range(N)]
        a_table[0][0]=0
        for city in range(1,N):
            time = a_table[0][city-1]
            if time == imp:
                a_table[0][city] = imp
            else:
                n = 0
                while businfo[city-1][0] + n * businfo[city-1][1] < time:
                    n += 1
                new_time = businfo[city-1][0] + n * businfo[city-1][1] + businfo[city-1][2]
                if new_time>Tf:
                    a_table[0][city]=imp
                else:
                    a_table[0][city]= new_time

        # sight see
        for city in range(0, N-1):
            time = a_table[0][city]

            if time == imp or time + Ts > Tf:
                a_table[0 + 1][city] = imp
            else:
                a_table[0 + 1][city] = time + Ts
        #print(a_table)
        res = work(N,Ts,Tf,businfo,a_table)
        print("Case #{}: {} \n".format(t0 + 1, res))
        output.write("Case #{}: {} \n".format(t0 + 1, res))

#ss== 0 means didn't sightsee last time
def work(N,Ts,Tf,businfo,a_table):
    for ss in range (1,N):




        #don't sight see
        for city in range(ss,N):
            time = a_table[ss][city-1]
            if time == imp:
                a_table[ss][city] = imp
            else:
                n = 0
                while businfo[city-1][0] + n * businfo[city-1][1] < time:
                    n += 1
                new_time = businfo[city-1][0] + n * businfo[city-1][1] + businfo[city-1][2]
                if new_time>Tf:
                    a_table[ss][city]=imp
                elif a_table[ss][city]==-1 or a_table[ss][city]==imp or a_table[ss][city]>new_time:
                    a_table[ss][city]= new_time
        # sight see
        if ss+1==N:
            break
        for city in range(ss, N-1):
            time = a_table[ss][city]

            if time == imp or time + Ts > Tf:
                a_table[ss + 1][city] = imp
            else:
                a_table[ss + 1][city] = time + Ts

    print(a_table)
    for i in range(len(a_table)-1,-1,-1):
        if a_table[i][-1] != imp and a_table[i][-1]!=-1:
            return i
    return imp

main("A-small-attempt2.in")