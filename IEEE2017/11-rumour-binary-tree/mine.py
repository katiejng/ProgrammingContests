t = int(input())
for _ in range(t):
    a, b = map(int, input().split())
    count = 0
    while a!=b:
        #print(a,b)
        if a<b:
            a, b = b, a
        #a is larger
        a = a//2
        count +=1
    print(count)
