class Node:
    def __init__(self,x,y,rows,cols, alive):
        self.x = x
        self.y = y
        self.alive = alive
        self.neighbors=[\
            ((x-1)%rows,(y-1)%cols),\
            ((x-1)%rows,(y+1)%cols),\
            ((x-1)%rows,(y)%cols),\
            ((x)%rows,(y-1)%cols),\
            ((x)%rows,(y+1)%cols),\
            ((x+1)%rows,(y)%cols),\
            ((x+1)%rows,(y-1)%cols),\
            ((x+1)%rows,(y+1)%cols)]
        self.nextState=0

    def getNextState(self,board):
        alive_neighbor=0
        for neighbor in self.neighbors:
            if board[neighbor[0]][neighbor[1]].alive:
                alive_neighbor+=1
        if self.alive:
            if alive_neighbor<2:
                self.nextState=0
            elif alive_neighbor==2 or alive_neighbor==3:
                self.nextState=1
            else:
                self.nextState=0
        else:
            if alive_neighbor==3:
                self.nextState=1

    def switchState(self):
        self.alive=self.nextState

def doStep(board):
    for row in board:
        for col in row:
            col.getNextState(board)

    for row in board:
        for col in row:
            col.switchState()

def print_board(board):

    for row in board:
        for col in row:

            print('*' if col.alive else '-',end="")
        print()
    print()

def main(board, steps, row, col):

    #print_board(board)

    for i in range(steps):
        doStep(board)
    print_board(board)



if __name__ == '__main__':
    row, col, steps = map(int, input().split())

    board = []
    for r in range(row):
        a_column = []
        for c in range(col):
            a_column.append((Node(r,c,row,col,0)))
        board.append(a_column)
    for i in range(row):
        line = input().strip()
        for charIndex in range(len(line)):
            if line[charIndex] == "*":
                board[i][charIndex].alive="*"
    #print (row, col, steps)
    main(board, steps,row,col)
