import math
def getChord(r,angle):
    angle = angle * math.pi/180
    h=r * (1- math.cos(angle/2))
    S = 2 * math.sqrt(2* r * h - h**2)
    return S



if __name__ == '__main__':
    r = int(input())
    letters = {}
    chords = {}
    for i in range(26):
        a, b = input().split()
        letters[a]=float(b)
    #print(getChord(10,40))
    #print(letters)

    sentence = ''.join(filter(str.isalpha, input())).upper()

    #print(sentence)
    res = r
    #print(sentence[0], str(res))
    curangle = letters[sentence[0]]
    for letterIndex in range(1,len(sentence)):
        newangle = letters[sentence[letterIndex]]
        diffangle = curangle-newangle
        diffangle = abs((diffangle + 180) %360 -180)
        #print(curangle, newangle, diffangle)
        chord = getChord(r, diffangle)
        res+=chord
        #print("Chord: ", chord, r, diffangle)
        curangle = newangle
        #print(sentence[letterIndex], str(res))
    print(math.ceil(res))
