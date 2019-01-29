def triangleOrNot(a, b, c):
    res = []

    for triangleIndex in range(len(a)):
        triangle = [a[triangleIndex], b[triangleIndex], c[triangleIndex]]
        atriangle = sorted(triangle)
        print(atriangle)
        if (atriangle[0]+atriangle[1]<=atriangle[2]):
            res.append("No")
        else:
            res.append("Yes")
    return res

print(triangleOrNot([7,10,7],[2,3,4],[2,7,4]))