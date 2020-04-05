import sys


def main():
    T = int(input())
    for t0 in range(T):
        string = list(map(int, list(input())))

        count = 0
        result = []
        for i in range(len(string)):
            current = string[i]
            if (current == count):
                pass
            elif(current > count):
                # insert the difference of brackets
                numBrackets = (current-count)
                result.append(numBrackets * "(")
                count = current

            else:
                numBrackets = abs(current-count)
                result.append(numBrackets * ")")
                count = current

            result.append(str(current))

        result.append(")" * count)

        print("Case #{}: {}".format(t0 + 1, "".join(result)))


main()
