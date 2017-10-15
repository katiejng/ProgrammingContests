

def getNextState(curstate, alive_neighbor):

    nextState= curstate
    if curstate:  #alive
        if alive_neighbor<2:
            nextState=0
        elif alive_neighbor==2 or alive_neighbor==3:
            nextState=1
        else:
            nextState=0
    else:
        if alive_neighbor==3:
            nextState=1
        else:
            nextState=0
    return nextState


def doStep(board):
    row = len(board)
    col = len(board[0])
    #print(row, col)
    counters = [ [ 0 for j in range(col)] for i in range(row)]
    #print(counters)
    #check right
    for x in [-1,0,1]:
        for y in [-1,0,1]:
            if (not(x==0 and y==0)):
                for curx in range (row):
                    for cury in range(col):
                        if board[(curx+x)%row][(cury+y)%col]=="*":
                            counters[curx][cury]+=1


    for curx in range(row):
        for cury in range(col):
            counters[curx][cury]=getNextState(1 if board[curx][cury]=="*" else 0,counters[curx][cury])
    
    for curx in range(row):
        a_str = ""
        for cury in range(col):
            a_str += "*" if counters[curx][cury] else "-"
        board[curx]=a_str

def print_board(board):

    for row in board:
        print(row)
    print()

def main(board, steps, row, col):

    #print_board(board)

    for i in range(steps):
        doStep(board)
    print_board(board)



if __name__ == '__main__':
    row, col, steps = map(int, input().split())

    board = []
    for i in range(row):
        line = input().strip()
        board.append(line)
    #print (row, col, steps)
    main(board, steps,row,col)
