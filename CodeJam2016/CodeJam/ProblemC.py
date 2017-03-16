def main(filename):
    file = open(filename,'r')
    test_cases = int(file.readline())
    output = open("OutputFileC.out",'w')
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



main("C-sample.txt")