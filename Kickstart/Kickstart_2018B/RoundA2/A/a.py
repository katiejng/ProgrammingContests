inputFile = "A-small-attempt6.in"
outputFile = "A-small-attempt6.out"

with open(inputFile) as f:
    with open(outputFile, "w") as g:
        t = int(f.readline().strip())
        for i in range(t):
            sum = 0
            oldn = int(f.readline().strip())
            n = oldn
            index = 0
            tempn = n
            start = -1
            result = ""
            # IGNORE STARTING EVEN NUMBERS
            for index in range(len(str(n))):
                currentNumber = int(str(n)[index])
                if (currentNumber % 2 == 0):
                    continue
                else:
                    #start here
                    start = index
                    break
            if start == -1:
                result = str(n)
            else:
                result = str(n)[:start]
                currentNumber = int(str(n)[start])

                if not(currentNumber %2 == 0):
                    a = (currentNumber+1)*(10**(len(str(n))-start-1))
                    b = int(str(currentNumber-1)+('8'*(len(str(n))-(start+1))))
                    #print(a,str(n)[start:], b)
                    numberToCompare = int(str(n)[start:])
                    if (currentNumber == 9):
                        result += str(b)
                    elif abs(numberToCompare-a)>abs(numberToCompare-b):
                        #  choose b
                        result += str(b)
                    else:
                        result += str(a)


            print("RES: " , result)



            g.write("Case #{}: {}\n".format(i + 1, abs(oldn-int('0'+result))))


