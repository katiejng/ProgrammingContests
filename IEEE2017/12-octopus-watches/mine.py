r,c = map(int, input().split())
watches = []
for _ in range(r):
    watches.append(list(map(int,input().split())))


for arow in range(r):
    for acol in range(c):
        watches[arow][acol] %= 3


print(r,c)
for line in watches:
    print(line)
