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

def TripleSort(a_list):
    done = False
    while not done:
        done = True
        for i in range (len(a_list)-2):
            if a_list[i]>a_list[i+2]:
                done = False
                a_list[i+2], a_list[i] = a_list[i], a_list[i+2]

    return (a_list)

def main():
    T = int(input())
    for t0 in range(T):
        n = int(input())
        numbers = list(map(int, input().split()))

        # two lists
        a_list = []
        b_list = []

        for i in range(len(numbers)):
            if (i%2 == 0):
                a_list.append(numbers[i])
            else:
                b_list.append(numbers[i])
        #print(a_list, b_list)

        a_list = sorted(a_list)
        b_list = sorted(b_list)
        currentValue = a_list[0]
        resultIndex = -1

        for index in range(len(a_list)):
            #print(a_list[index])

            #print(b_list[index])
            if (a_list[index]< currentValue):
                # failed
                resultIndex = index*2 -1
                break
            else:
                currentValue = a_list[index]

            if index>=len(b_list):
                break
            if (b_list[index] < currentValue):
                # failed
                resultIndex = index * 2
                break
            else:
                currentValue = b_list[index]







        if resultIndex == -1:
            resultIndex = "OK"



        print ("Case #{}: {}".format(t0+1, resultIndex))

main()