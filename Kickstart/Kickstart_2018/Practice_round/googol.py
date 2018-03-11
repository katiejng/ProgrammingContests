N  = int(input())

for i in range(N):
    pointer = pow(10,100)/2
    value = 0
    K = int(input())
    while (K == pointer):
        if (K<pointer):
            pointer = pointer/2
