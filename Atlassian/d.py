# Complete the function below.

def getMostVisited(n, sprints):
    visited = [0]*(n+1)
    for i in range(len(sprints)-1):
        j = i+1
        if j>i:
            for k in range(i,j):
              visited[k]+=1
        else:
            for k in range(j, i):
                visited[k] += 1

    print(visited)
    return(visited.index(max(visited)))


