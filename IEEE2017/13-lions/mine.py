

r,c,k = map(int, input().split())
lions = []
for _ in range(k):
    lions.append(list(map(int,input().split())))



print(r,c,k)
for line in lions:
    print(line)


#calculate territory for each lion then determine who is in there
