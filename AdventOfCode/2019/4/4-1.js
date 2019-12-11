const input = require('./4-1-input');

function hasDuplicates(array) {
    count = 1;
    for (let i = 1; i < array.length; i++){
        if (array[i] === array[i-1]){
            count += 1
        }else{
            if (count == 2){
                return true
            }
            count = 1;
        }
    }
    if (count == 2){
        return true
    }
    return false
}

function increment(array){
    var i = 5;
    array[i] += 1
    while (array[i] >= 10){
        i --
        array[i] += 1
    }
    while (i < array.length-1){
        array[i+1] = array[i]
        i++
    }
    return array
}

const parsedInput = input.value.split('-').map(x=> parseInt(x))


var sum = 0;
var start = parsedInput[0].toString().split('').map(x => parseInt(x))


let found = 0;
for (let i = 1; i < start.length; i++){
    if (start[i]<start[i-1] || found>0){
        found = start[i-1]
        start [i] = found
    }
}

while (parseInt(start.join('')) <= parsedInput[1]){

    if ( hasDuplicates(start)){
        sum += 1;
    }
    start = increment(start)
}

console.log('sum', sum)