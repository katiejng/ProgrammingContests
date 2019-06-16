
fact_sums = [1]*1001
fact = 1
for i in range(2,1001):
    fact = fact*i
    numbers = list(str(fact))
    for j in range(len(numbers)):
        numbers[j] = int(numbers[j])

    fact_sums[i] = sum(numbers)


T = int(input())
for i in range(T):
    N = int(input())
    print(fact_sums[N])