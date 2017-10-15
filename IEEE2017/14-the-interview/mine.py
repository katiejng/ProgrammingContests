# bottom-up, dynamic programming solution using a single array

def num_subsequences(seq, sub):
    m, n = len(seq), len(sub)
    table = [0] * n
    maxtempres = 0
    for i in range(m):
        previous = 1
        tempres = 0
        for j in range(n):
            current = table[j]
            if seq[i] == sub[j]:
                table[j] += previous
                tempres+=1
                if tempres>maxtempres:
                    maxtempres=tempres
                print("HERE:{}".format(maxtempres))
            previous = current

    return table[n-1] if n else maxtempres

def largest_subsequence(r,c,stra,strb):
    table = [ [ 0 for j in range(c+1)] for i in range(r+1)]
    #print(table)
    for i in range(r):
        for j in range(c):
            table[i+1][j]=max(table[i+1][j],table[i][j])
            if stra[i] ==strb[j]:
                table[i+1][j+1]=max(table[i+1][j+1],table[i][j]+1)
    possible_values = table[r]+[table[x][-1] for x in range(r)]
    #print(possible_values)
    return max(possible_values)

the_string = input().strip()[::-1]
t = int(input().strip())
for _ in range(t):
    compare_string = input().strip()[::-1]
    res =largest_subsequence(len(the_string), len(compare_string), the_string, compare_string)

    print(res)
