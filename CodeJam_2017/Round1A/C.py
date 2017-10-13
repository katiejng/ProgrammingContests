import math
Dh0 = Da0 = Kh0 = Ka0 = B = D = 0


def simulate(d,b):

    Dh = Dh0
    Da = Da0
    Kh = Kh0
    Ka = Ka0
    cured = False

    num_turns = d+b
    #while the knight is still alive
    #remember the knight attacks every action


    #debuff d times
    for _ in range(d):
        #print("Debuff")
        Ka-=D
        if Ka<=0:
            Ka = 0
        Dh-=Ka  # Knight's attack
        if Dh<=0:
            #print("Dead")
            return -1
        if Da>=Kh:
            return num_turns+1
        if Dh<=Ka:
            if cured:
                #print("Cured Twice in a row")
                return -1
            #print("Cured")
            Dh = Dh0-Ka
            num_turns+=1
            cured = True
        else:
            cured = False
        #cure if dying , if you cure twice in a row, IMP
    #buff b times
    for _ in range(b):
        #print("Buffed")
        Da+=B
        Dh-=Ka
        if Dh<=0:
            #print("Dead")
            return -1
        if Da>=Kh:
            return num_turns+1
        if Dh<=Ka:
            if cured:
                #print("cured twice in a row")
                return -1
            #print("Cured")
            Dh = Dh0-Ka
            num_turns+=1
            cured = True
        else:
            cured = False
        #cure if dying, if you cure twice in a row, IMP

    while Kh>0:
        #print("Attack", Dh,Da, Kh, Ka)
        Kh-=Da
        Dh-=Ka
        num_turns+=1
        if Kh<=0:
            return num_turns
        if Dh <= 0:
           #print("Dead")
            return -1
        if Da>=Kh:
            return num_turns+1
        if Dh <= Ka:

            if cured:
                #print("cured twice in a row")
                return -1

            #print("Cured")
            Dh = Dh0-Ka
            num_turns += 1
            cured = True
        else:
            cured = False
    #attack

    return num_turns
    #return the number of actions

def main(filename):
    file = open(filename, 'r')
    t = int(file.readline())
    output = open(filename + ".out", 'w')
    res = 0
    for t0 in range(0, t):
        global Dh0, Da0,Kh0, Ka0, B,D
        Dh0, Da0, Kh0, Ka0, B, D = map(int, file.readline().split())
        #print(Dh0, Da0,Kh0, Ka0, B,D)
        num_steps = -1
        #If the knight kills you even if you debuff or attack, IMP
        if Dh0<=Ka0-D:
            num_steps = "IMPOSSIBLE"
            print("IMP")
        else:
            #Simulate d = Debuff, b = Buff
            for d in range(100):
                for b in range(100):
                    #print("NEXT SIMULATION", d,b)
                    steps = simulate(d,b)
                    if steps == -1:
                        continue
                    elif steps<num_steps or num_steps == -1:
                        num_steps = steps
                        #print(b,d,num_steps)


        if num_steps == -1:
            num_steps = "IMPOSSIBLE"
        print("Result: ",num_steps)
        output.write("Case #{}: {}\n".format(t0 + 1,num_steps))
    file.close()
    output.close()


main("C-small-practice.in")