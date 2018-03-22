# Complete the function below.

def getDegree(elements):
    sums = {}
    for item in elements:
        if item in sums:
            sums[item] += 1
        else:
            sums[item] = 1

    degrees = []
    degree = 0
    for item in sums:
        if degree < sums[item]:
            degree = sums[item]
            degrees = [item]
        elif degree == sums[item]:
            degrees.append(item)
    return(degree, degrees)

def degreeOfArray(elements):

    (degree, degrees) = getDegree(elements)
    # got degree now get subarrays
    #must be length 3 or higher, should aim to include as many degrees in as possible
    if degree == 1:
        return 1
    # else length of subarray must be 2 or higher
    result = -1
    for value in degrees:
        indexes = [i for i, j in enumerate(elements) if j == value]
        lengthOfSubArray = indexes[-1]-indexes[0]+1
        print(lengthOfSubArray)
        if result == -1:
            result = lengthOfSubArray
        elif result> lengthOfSubArray:
            result = lengthOfSubArray
    return result





elements = [1,2,3,4,5,6,1,2,6,1,6,1,6]
print(degreeOfArray(elements))