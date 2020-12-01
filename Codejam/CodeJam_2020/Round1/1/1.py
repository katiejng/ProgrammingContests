import sys

def getFirstChar(strin):
    if (len(strin ) == 0):
        return ""
    return strin[0]

def getNextChar(characters):

    isForced = False
    forcedChar = ""
    for pattern in characters:
        if (len(pattern) == 0):
            continue
        if getFirstChar(pattern) != "*":
            if (isForced and forcedChar != getFirstChar(pattern)):
                return False
            isForced = True
            forcedChar = getFirstChar(pattern)

    if (isForced):
        return forcedChar
    # next letter is optional
    nextChar = ""

    for character in characters:
        if (len(character) > 1):
            nextChar = characters[0][1]
    return nextChar


def checkEmpty(patterns, N):
    allEmpty = True
    allAsterisk = True

    for index in range(N):
        if (len(patterns[index]) > 0 and getFirstChar(patterns[index]) != "*"):
            allAsterisk = False
        
        if (len(patterns[index]) > 0):
            allEmpty = False

    return allEmpty, allAsterisk
    
def doThing(patterns, N):
    result = []

    while(True):
        patterns = sorted(patterns, key=len)
        patterns.reverse()


        allEmpty, allAsterisk = checkEmpty(patterns, N)

        if (allAsterisk):
            ### START FROM THE RIGHT
            break

        if (allEmpty):
            ### FINISHED
            break

        nextChar = getNextChar(patterns)
        if (nextChar is False):
            result = ["*"]
            break
        result.append(nextChar)

        for index in range(N):
            if (getFirstChar(patterns[index]) == nextChar):
                patterns[index] = patterns[index][1:]
    return (patterns, result)

def reversePatterns(patterns):
    # REVERSE PATTERNS
    newPatterns = []
    for pattern in patterns:
        newPatterns.append(pattern[::-1])
    return newPatterns


def main():
    T = int(input())
    for t0 in range(T):
        N = int(input())
        patterns = []
        for n0 in range(N):
            patterns.append(input())


        resultStart = []
        resultsEnd = []


        while (True):
            # print("---------" , patterns, resultStart, resultsEnd)

            # FORWARD PASSS
            patterns, result = doThing(patterns, N)
            resultStart.extend(result)

            allEmpty, allAsterisk = checkEmpty(patterns, N)

            if (allEmpty):
                # print("EMPTY")
                break

            if (allAsterisk):
                for index in range(N):
                    patterns[index] = patterns[index][1:]
                    break

            patterns = reversePatterns(patterns)

            # print("---------" , patterns, resultStart, resultsEnd)

            # REVERSE
            patterns, result = doThing(patterns, N)
            
            resultsEnd.extend(result)

 

            allEmpty, allAsterisk = checkEmpty(patterns, N)

            if (allEmpty):
                # print("EMPTY")
                break

            if (allAsterisk):
                for index in range(N):
                    patterns[index] = patterns[index][1:]
                    break


            if (resultStart.count("*") > 0 or resultsEnd.count("*") > 0):
                resultStart = ["*"]
                resultsEnd = []
                # print("NO SOL")
                break

            patterns = reversePatterns(patterns)


        resultsEnd.reverse()
        print("Case #{}: {}{}".format(t0 + 1, "".join(resultStart), "".join(resultsEnd)))


main()
