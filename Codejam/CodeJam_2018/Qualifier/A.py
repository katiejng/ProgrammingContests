import sys

def solve(a, b):
  m = (a + b) // 2
  print(m)
  sys.stdout.flush()
  s = input()
  if s == "CORRECT":
    return
  elif s == "TOO_SMALL":
    a = m + 1
  else:
    b = m - 1
  solve(a, b)

def getValues(aString):
    values = []
    power = 1
    for i in range(len(aString)):
        if aString[i] == "C":
            power *= 2
            #print(power)
        else:
            values.append(power)
    return values


def main():
    T = int(input())
    for t0 in range(T):
        a, b = input().split()
        a = int(a)
        #print(a,b)

        values = getValues(b)
        #print(values)
        count = 0
        if 1*len(values)>a:
            print ("Case #{}: IMPOSSIBLE".format(t0+1))
            continue
        while (sum(values)>a):
             # halve the biggest number
            #print("half")
            maxValue = max(values)
            maxIndex = values.index(maxValue)
            values[maxIndex] = maxValue/2
            count+=1
        print ("Case #{}: {}".format(t0+1, count))

main()