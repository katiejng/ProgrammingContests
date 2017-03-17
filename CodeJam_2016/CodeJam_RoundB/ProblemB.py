def main(filename):
    file = open(filename,'r')
    test_cases = int(file.readline())
    output = open("OutputFileB.out",'w')
    for i in range(1,test_cases+1):
        R_C = file.readline()
        R_C = R_C.split()
        R = int(R_C[0])
        C = int(R_C[1])
        board = []

        for i in range(R):
            board.append(file.readline().split())

        total_increased_height = TOTAL(board,R,C)
        output.write("Case #{}: {} \n".format(i,total_increased_height))

def TOTAL(board,R,C):
    total = 0
    changes = True
    while changes:
        changes = False
        for r in range(1,R-1):
            for j in range(1,C-1):
                if board[r][j]<board[r-1][j] and board[r][j]<board[r+1][j] and board[r][j]<board[r][j-1] and board[r][j]<board[r][j+1]:
                    prev = board[r][j]
                    board[r][j]= min(board[r-1][j],board[r+1][j],board[r][j-1],board[r][j+1])
                    changes = True
                    total += int(board[r][j])-int(prev)
    print(board)
    return total




main("B-sample.txt")