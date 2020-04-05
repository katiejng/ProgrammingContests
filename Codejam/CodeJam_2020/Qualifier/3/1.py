import sys


def main():
    T = int(input())
    for t0 in range(T):
        N = int(input())
        schedule = []
        for n0 in range(N):
            result = list(map(int, input().split()))
            schedule.append([result, n0])

        schedule.sort()

        C = 0
        J = 0

        result = [0]*N
        for i in range(len(schedule)):
            # for each schedule item
            appointment, index = schedule[i]
            start, finish = appointment
            # impossible if C and J are both busy
            if (C > start and J > start):
                result = "IMPOSSIBLE"
                break
            # give task to whoever finished earlier
            elif (C <= start):
                result[index] = "C"
                C = finish
            else:
                result[index] = "J"
                J = finish

        print("Case #{}: {}".format(t0 + 1, "".join(result)))


main()
