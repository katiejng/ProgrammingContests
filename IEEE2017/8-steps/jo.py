def recursive(n):
    if n in traversals:
        return traversals[n]
    traversals[n] = recursive(n - 1) + recursive(n - 2)
    return traversals[n]

t = int(raw_input().strip())

traversals = dict()
traversals[0] = 1
traversals[1] = 1

for _ in range(t):
    n = int(raw_input().strip())
    benchmark = max(traversals.keys())
    while n - benchmark > 500:
        recursive(benchmark + 500)
        benchmark = max(traversals.keys())
    print(recursive(n))
