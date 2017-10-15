modulo = 1000000007
facts = {}
def factn(n,end=0):
    if end==0:
        if n in facts:
            return facts[n]
    res = 1
    for i in range(n,end,-1):
        res= (res *i)
    if end == 0:
        facts[n]=res
    return res
def fact(n):
    if n in facts:
        return facts[n]
    res = 1
    for i in range(1,n+1,1):
        res= (res *i)
        facts[i]=res
    return res

def main():
    t = int(input())
    for t0 in range(0,t):
        a,b,c = map(int, input().split())
        if (a==1):
            res = 1
        else:
            if c>b//2:
                top = int(factn(b,c))
                bot= int(fact(b-c))
            else:
                top = int(factn(b,b-c))
                bot = int(fact(c))

            res = a**(int(top/bot))%modulo

        print(res)


main()
