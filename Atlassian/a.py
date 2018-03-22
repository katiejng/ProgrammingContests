# Complete the function below.

def rearrange(elements):
    elements = sorted(elements)
    #print(elements)
    newElements = []
    for item in elements:
        binaryVal = "{0:b}".format(item)
        newElements.append(( binaryVal.count('1'), item))
    newElements = sorted(newElements)
    result = []
    for item in newElements:
        result.append(item[1])

    return result


elements = [1,2,3]
print(rearrange(elements))