const input = require("./5-2-input");

const parsedInput = input.value.split(",").map(x => parseInt(x));
function parseInput(command) {
  // check for
  if (command == 3 || command == 4) {
    return {
      command
    };
  }
  const splitCommand = command
    .toString()
    .split("")
    .map(x => parseInt(x));
  while (splitCommand.length < 5) {
    splitCommand.splice(0, 0, 0);
  }
  return {
    command: splitCommand[4],
    values: [splitCommand[2], splitCommand[1], splitCommand[0]]
  };
}

function getValues(start, command, n) {
  let values = [];
  let indices = [];

  for (let i = 0; i < n; i++) {
    let index = parsedInput[start + i + 1];
    indices.push(index);
    values.push(command.values[i] ? index : parsedInput[index]);
  }
  return {
    values,
    indices
  };
}

function handleAddition(start, command) {
  const res = getValues(start, command, 3);
  parsedInput[res.indices[2]] = res.values[0] + res.values[1];
  return start + 4;
}

function handleMultiplication(start, command) {
  const res = getValues(start, command, 3);
  parsedInput[res.indices[2]] = res.values[0] * res.values[1];
  return start + 4;
}

function handleSave(start) {
  const firstIndex = parsedInput[start + 1];
  const input = 5;
  parsedInput[firstIndex] = input;
  return start + 2;
}

function handleOutput(start) {
  const firstIndex = parsedInput[start + 1];
  const output = parsedInput[firstIndex];
  console.log("output", output);
  return start + 2;
}

function handleJumpIfTrue(start, command) {
  const res = getValues(start, command, 2);

  if (res.values[0] !== 0) {
    return res.values[1];
  }
  return start + 3;
}

function handleJumpIfFalse(start, command) {
  const res = getValues(start, command, 2);

  if (res.values[0] === 0) {
    return res.values[1];
  }
  return start + 3;
}

function handleLessThan(start, command) {
  const res = getValues(start, command, 3);

  if (res.values[0] < res.values[1]) {
    parsedInput[res.indices[2]] = 1;
  } else {
    parsedInput[res.indices[2]] = 0;
  }

  return start + 4;
}

function handleEquals(start, command) {
  const res = getValues(start, command, 3);

  console.log("eq", res);
  if (res.values[0] == res.values[1]) {
    parsedInput[res.indices[2]] = 1;
  } else {
    parsedInput[res.indices[2]] = 0;
  }

  return start + 4;
}

function processList() {
  let i = 0;
  while (i < parsedInput.length) {
    const theCommand = parseInput(parsedInput[i]);
    let nextI;
    switch (theCommand.command) {
      case 1:
        nextI = handleAddition(i, theCommand);
        break;
      case 2:
        nextI = handleMultiplication(i, theCommand);
        break;
      case 3:
        nextI = handleSave(i);
        break;
      case 4:
        nextI = handleOutput(i);
        break;
      case 5:
        nextI = handleJumpIfTrue(i, theCommand);
        break;
      case 6:
        nextI = handleJumpIfFalse(i, theCommand);
        break;
      case 7:
        nextI = handleLessThan(i, theCommand);
        break;
      case 8:
        nextI = handleEquals(i, theCommand);
        break;
      case 99:
        break;
    }
    i = nextI;
  }
}

processList();

console.log(parsedInput.join());
