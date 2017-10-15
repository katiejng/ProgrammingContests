def fib_to(n):
    fibs = [1, 2]
    for i in range(2, n+1):
        fibs.append((fibs[-1] + fibs[-2])%10)
    return fibs

def get_pisano_period(m):
    a =0
    b=1
    c=a+b
    for i in range (m*m):
        c = (a+b)%m
        a=b
        b=c
        if (a==0 and b==1):
            return i+1

def get_large_fib(n,m):
    rem = n % get_pisano_period(m)
    first = 0
    second = 1
    res = rem
    for i in range(rem):
        res = (first+second)%m
        first = second
        second = res
    return res % m

fibs = fib_to(59)
for i in range(len(fibs)):
    print("{}: {}".format(i,fibs[i]))
t = int(input())
for i in range(t):
    d = int(input())
    print(fibs[d%60-1])
