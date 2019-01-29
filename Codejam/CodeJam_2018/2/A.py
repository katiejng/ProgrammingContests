
# def doIt(b,c):
#     # determine where each ball will go
#     grid = []
#     positions = [1]*c
#
#     for row in range(c+1):
#         print(positions)
#
#         nextPosition = positions[:]
#         grid.append(['.']*c)
#         if positions == b:
#             return grid, positions # done
#
#
#         # this algorithm assumes it starts with 1 in each spot :(
#         partitions = [0]*(c+1)  # get partitions
#         for i in range(1,c+1):
#             partitions[i] = b[i-1]+partitions[i-1]
#
#         newPositions = {}
#
#         for partIndex in range(len(partitions)-1):
#             print(partitions[partIndex], partitions[partIndex+1])
#             for index in range(partitions[partIndex], partitions[partIndex+1]):
#                 newPositions[index] = partIndex
#         print(newPositions)
#
#         # shift them
#         for column in range(c):
#             if newPositions[column] < column:
#
#                 # shift left
#                 grid[row][column] = '/'
#                 nextPosition[column-1] += positions[column]
#                 nextPosition[column] -= positions[column]
#             elif newPositions[column] > column:
#                 grid[row][column] = "\\"
#                 nextPosition[column+1] += positions[column]
#                 nextPosition[column] -= positions[column]
#             else:
#                 grid[row][column] = '.'
#         positions = nextPosition[:]
#
#
#
#     return grid, positions
#
#
#
#
#
#     # for partIndex in range(len(partitions)-1):
#     #     start = partitions[partIndex]
#     #     end = partitions[partIndex+1]
#     #     if(start == end):
#     #         continue
#     #     # shift over ones that need to be moved
#     #     mainSpot = 0
#     #
#     #     for position in range(partitions[partIndex], partitions[partIndex+1]):
#     #         if b[position]!= 0:
#     #             mainSpot = position
#     #     print("HERE", mainSpot)
#     #
#     #     # perform the shifting
#     #     moves = [0]*3
#     #     moves[1] = b[mainSpot]
#     #
#     #     if mainSpot > start:
#     #         # shift left that many
#     #         moves[0] = moves[1]-(mainSpot-start)
#     #         moves[1] = moves[1]-moves[0]
#     #     if mainSpot < end:
#     #         moves[2] = moves[1]-(end - mainSpot)
#     #         moves[1] = moves[1] = moves[2]
#     #     print("MOVES", moves)
#
#     print("newpostitions", newPositions)
#
#
#
#
#


def doIt(b,c):
    grid = []
    positions = [1]*c
    for row in range(c+1):
        grid.append(['.']*c)
        if positions == b:
            return grid, positions
        # determine final spots


        moveTo = {}
        tempPos = positions[:]
        nextPosition = positions[:]

        tempB = b[:]
        for posIndex in range(c):
            if tempPos[posIndex] > 0:
                for bIndex in range(c):
                    if tempB[bIndex] > 0:
                        moveTo[posIndex] = bIndex
                        tempB[bIndex]-=tempPos[posIndex]
                        tempPos[posIndex] = 0
                        break
        # print(moveTo)

        # shift
        for column in range(c):
            try:
                if moveTo[column] < column:
                        # shift left
                        grid[row][column] = '/'
                        nextPosition[column-1] += positions[column]
                        nextPosition[column] -= positions[column]
                elif moveTo[column] > column:
                    grid[row][column] = "\\"
                    nextPosition[column+1] += positions[column]
                    nextPosition[column] -= positions[column]
                else:
                    grid[row][column] = '.'
            except KeyError:
                grid[row][column] = '.'
        positions = nextPosition[:]


    return grid, positions







def main():
    T = int(input())
    for t0 in range(T):
        c = int(input())
        b = list(map(int, input().split()))
        # print(c, b)
        result = 1
        if (sum(b) != c):
            result = "IMPOSSIBLE"
        if (b[0] == 0 or b[c-1] == 0):
            print("Case #{}: {}".format(t0 + 1, "IMPOSSIBLE"))
            continue
        else:
            (grid, positions) =doIt(b,c)
            print("Case #{}: {}".format(t0 + 1, len(grid)))
            for row in grid:
                for item in row:
                    print(item, end="")
                print()


main()