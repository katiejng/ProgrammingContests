const input = require('./2-1-input');

const parsedInput = input.value.split(',').map(x => parseInt(x))
console.log(parsedInput.join())


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

processList();

console.log(parsedInput.join())