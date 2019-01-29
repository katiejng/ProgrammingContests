inputFile = "small.in"
outputFile = "small.out"

with open(inputFile) as f:
    with open(outputFile, "w") as g:
        t = int(f.readline().strip())
        for i in range(t):
            sum = 0
            n = int(f.readline().strip())
            for number in range(len(str(n))):
                a = int(str(n)[number]) * 10*(len(str(n))-1)
                print(n)
                b = int(str(n)[number])-1 * 10*(len(str(n))-1)
                print(a)
                print()


            g.write("Case #{}: {}\n".format(i + 1, n))