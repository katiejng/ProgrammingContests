import sys


def main():
    T = int(input())
    for t0 in range(T):
        [N, K] = list(map(int, input().split()))
        if (K <= 0 or K % N != 0 or K > N*N):
            print("Case #{}: {}".format(t0 + 1, "IMPOSSIBLE"))
            continue

        starting = (K // N) - 1

        x = [[0] * N for i in range(N)]
        for r0 in range(N):
            temp = starting
            for c0 in range(N):
                x[r0][c0] = str(temp + 1)
                temp = ((temp - 1) % N)
            starting = ((starting + 1) % N)

        print("Case #{}: {}".format(t0 + 1, "POSSIBLE"))

        for r0 in range(N):
            print(" ".join(x[r0]))


main()
