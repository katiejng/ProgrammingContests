const input = require('./2-2-input');

var parsedInput;

function getParsedList(){
    parsedInput = input.value.split(',').map(x => parseInt(x))
}

function handleAddition(start){
    const firstIndex = parsedInput[start+1]
    const secondIndex = parsedInput[start+2]
    const thirdIndex = parsedInput[start+3]
    parsedInput[thirdIndex] = parsedInput[firstIndex] + parsedInput[secondIndex]
}

function handleMultiplication(start){
    const firstIndex = parsedInput[start+1]
    const secondIndex = parsedInput[start+2]
    const thirdIndex = parsedInput[start+3]
    parsedInput[thirdIndex] = parsedInput[firstIndex] * parsedInput[secondIndex]
}

function processList(){
    for (let i = 0; i<parsedInput.length; i += 4){
        switch (parsedInput[i]){
            case 1:
                handleAddition(i)
                break;
            case 2:
                handleMultiplication(i)
                break;
            case 99:
                break;
        }
    }    
}
const goal = 19690720


for (let i = 0; i <=99; i++){
    for (let j = 0 ; j<= 99; j++){

        getParsedList();
        parsedInput[1] = i;
        parsedInput[2] = j;
        processList();
        if (parsedInput[0] == goal){
            console.log(i, j, parsedInput[0], 100 * i + j)
            break;
        }
    }
}


// console.log(parsedInput[0], parsedInput[0] == goal)