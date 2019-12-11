const input = require('./5-1-input');

const parsedInput = input.value.split(',').map(x => parseInt(x))

function parseInput(command){
    // check for 
    if (command == 3 || command == 4){
        return {
            command
        }
    }
    const splitCommand = command.toString().split('').map(x => parseInt(x))
    while (splitCommand.length < 5){
        splitCommand.splice(0, 0, 0)
    }
    return {
        command: splitCommand[4],
        values: [splitCommand[2], splitCommand[1], splitCommand[0]],
        first: splitCommand[2],
        second: splitCommand[1],
        third: splitCommand[0],
    }
}

function getValues(start, command, n){
    let values = []
    let indices = []

    for (let i = 0; i < n; i ++){
        let index = parsedInput[start+i+1]
        indices.push(index);
        values.push(command.values[i] ? index :  parsedInput[index])
    }
    return {
        values,
        indices
    }
}


function handleAddition(start, command){
    const res = getValues(start, command, 3)
    parsedInput[res.indices[2]] = res.values[0] + res.values[1]
    return 4;
}

function handleMultiplication(start, command){
    const res = getValues(start, command, 3)
    parsedInput[res.indices[2]] = res.values[0] * res.values[1]
    return 4;
}

function handleSave(start){
    const firstIndex = parsedInput[start+1]
    const input = 1
    parsedInput[firstIndex] = input
    return 2;
}

function handleOutput(start){
    const firstIndex = parsedInput[start+1]
    const output = parsedInput[firstIndex]
    console.log('output', output)
    return 2;
}

function processList(){
    let i = 0;
    while(i<parsedInput.length){
        let incAmount;
        
        const theCommand = parseInput(parsedInput[i])

        switch (theCommand.command){
            case 1:
                incAmount = handleAddition(i, theCommand)
                break;
            case 2:
                incAmount = handleMultiplication(i, theCommand)
                break;
            case 3:
                incAmount = handleSave(i)
                break;
            case 4:
                incAmount = handleOutput(i)
                break;
            case 99:
                break;
        }
        i += incAmount;
    }    
}

processList();

console.log(parsedInput.join())