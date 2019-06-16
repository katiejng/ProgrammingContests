def main():

    with open('2.out', 'w+') as f:
        T = int(input())

        for t0 in range(T):
            line = input()

            if (len(line) == 1):
                printRes(f, t0, "Y")
                continue
            if (line.count(".") == 0):
                printRes(f, t0, "N")
                continue
            if (line.count("B")>=2):
                printRes(f, t0, "Y")
                continue
            if (line.count(".") > line.count("B")):
                printRes(f, t0, "N")
                continue
            printRes(f, t0, "Y")


def printRes(f, t0, result):
    f.write("Case #{}: {}\n".format(t0 + 1, result))


main()