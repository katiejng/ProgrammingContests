import queue
class Group():
    def __init__(self,start, end):
        self.start = start
        self.end = end
        self.length = end-start

    def __gt__(self, group2):
        if self.length>group2.length:
            return False
        elif self.length == group2.length:
            if self.start<group2.start:
                return False
            else:
                return True
        else:
            return True



    def __str__(self):
        return "Start: {}\nEnd: {}\nLength: {}\n".format(self.start, self.end, self.length)

def work(t,N,K,Groups):
    while(K>0 and Groups):
        cur = Groups.get()
        #print(cur)
        split = (cur.start+cur.end)//2
        a = Group(cur.start,split)
        b = Group(split, cur.end)
        if (b.length > 1):
            Groups.put(b)
        if (a.length> 1):
            Groups.put(a)
        K-=1
    #print(cur)
    print("Case #{}: {} {}".format(t,cur.end-split-1,split-cur.start-1))





def main():
    with open("C-small-2-attempt0.in") as f:
        with open("output.txt", "w") as g:
            t = int(f.readline().strip())
            for i in range(1, t + 1):
                n,k = map(int,f.readline().split())
                Groups = queue.PriorityQueue()
                a = Group(0,n+1)
                Groups.put(a)


                work(i,n,k,Groups)




main()