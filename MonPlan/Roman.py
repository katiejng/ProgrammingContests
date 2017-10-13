
def num_to_rom(n):
    numbers = [1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000]
    letters = ["I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M"]
    numbers = numbers[::-1]
    letters = letters[::-1]
    i = 0
    res = ""

    while n>0:

        if n>=numbers[i]:
            res += letters[i]
            n-=numbers[i]

        else:
            i+=1

    return res

def num_to_rom2(n):
    numbers = [1000,500,100,50,10,5,1]
    letters = ["M","D","C","L","X","V","I"]
    res = ""
    i = 0
    while n>0:
        diff = i%2
        if i>=len(numbers)-1:
            res+=n*letters[i]
            break
        if n>=numbers[i]-numbers[i+2-diff]:
            # eg n = 1000, we want M
            # if n = 900, we want CM not DCCCC
            if n>=numbers[i]: # if n == 1000
                res+=letters[i]
                n-=numbers[i]
            else: # if n == 900
                res+=letters[i+2-diff]+letters[i]
                n-=numbers[i]-numbers[i+2-diff]
        else:
            i+=1
    return res
for i in range(1900,2000):
    print(num_to_rom2(i))

