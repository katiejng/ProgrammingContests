import input from "./in7-1";

enum CommandType {
  ADDITION = 1,
  MULTIPLICATION,
  SAVE,
  OUTPUT,
  JUMP_IF_TRUE,
  JUMP_IF_FALSE,
  LESS_THAN,
  EQUALS,
  EXIT = 99
}

const SpecialCommands = [CommandType.SAVE, CommandType.OUTPUT];

class Command {
  commandType: CommandType;
  immediate: Array<number>;
  values: Array<number> = [];
  indices: Array<number> = [];

  constructor(opcode: number) {
    this.parseInput(opcode);
  }

  parseInput(command: number) {
    if (command in SpecialCommands) {
      this.commandType = command;
      this.immediate = [0, 0, 0];
    }
    const splitCommand = command
      .toString()
      .split("")
      .map(x => parseInt(x));
    while (splitCommand.length < 5) {
      splitCommand.splice(0, 0, 0);
    }
    this.commandType = splitCommand[4];
    this.immediate = [splitCommand[2], splitCommand[1], splitCommand[0]];
  }

  parseOpcode(start: number, n: number) {
    for (let i = 0; i < n; i++) {
      let index = parsedInput[start + i + 1];
      this.indices.push(index);
      this.values.push(this.immediate[i] ? index : parsedInput[index]);
    }
  }
}

class IntCodeComputer {
  codes: Array<number>;
  input: Array<number>;
  output: number;
  inputCount: number = 0;

  constructor(input: Array<number>, codes: Array<number>) {
    this.codes = codes;
    this.input = input;
    this.processList();
  }

  handleAddition(start: number, command: Command) {
    command.parseOpcode(start, 3);
    parsedInput[command.indices[2]] = command.values[0] + command.values[1];
    return start + 4;
  }

  handleMultiplication(start: number, command: Command) {
    command.parseOpcode(start, 3);
    parsedInput[command.indices[2]] = command.values[0] * command.values[1];
    return start + 4;
  }

  handleSave(start: number, command: Command) {
    command.parseOpcode(start, 1);
    parsedInput[command.indices[0]] = this.input[this.inputCount];
    this.inputCount++;
    return start + 2;
  }

  handleOutput(start: number, command: Command) {
    command.parseOpcode(start, 1);
    this.output = parsedInput[command.indices[0]];
    return start + 2;
  }

  handleJumpIfTrue(start: number, command: Command) {
    command.parseOpcode(start, 2);

    if (command.values[0] !== 0) {
      return command.values[1];
    }
    return start + 3;
  }

  handleJumpIfFalse(start: number, command: Command) {
    command.parseOpcode(start, 2);

    if (command.values[0] === 0) {
      return command.values[1];
    }
    return start + 3;
  }

  handleLessThan(start: number, command: Command) {
    command.parseOpcode(start, 3);

    if (command.values[0] < command.values[1]) {
      parsedInput[command.indices[2]] = 1;
    } else {
      parsedInput[command.indices[2]] = 0;
    }

    return start + 4;
  }

  handleEquals(start: number, command: Command) {
    command.parseOpcode(start, 3);

    if (command.values[0] == command.values[1]) {
      parsedInput[command.indices[2]] = 1;
    } else {
      parsedInput[command.indices[2]] = 0;
    }

    return start + 4;
  }

  printOutput() {
    console.log("--------------\nOUTPUT: ", this.output, "\n--------------");
  }

  processList() {
    let i = 0;
    while (i < parsedInput.length) {
      const command = new Command(parsedInput[i]);
      let nextI;
      switch (command.commandType) {
        case CommandType.ADDITION:
          nextI = this.handleAddition(i, command);
          break;
        case CommandType.MULTIPLICATION:
          nextI = this.handleMultiplication(i, command);
          break;
        case CommandType.SAVE:
          nextI = this.handleSave(i, command);
          break;
        case CommandType.OUTPUT:
          nextI = this.handleOutput(i, command);
          break;
        case CommandType.JUMP_IF_TRUE:
          nextI = this.handleJumpIfTrue(i, command);
          break;
        case CommandType.JUMP_IF_FALSE:
          nextI = this.handleJumpIfFalse(i, command);
          break;
        case CommandType.LESS_THAN:
          nextI = this.handleLessThan(i, command);
          break;
        case CommandType.EQUALS:
          nextI = this.handleEquals(i, command);
          break;
        case CommandType.EXIT:
          break;
      }
      i = nextI;
    }
  }
}

function doIt(phases: Array<number>, args: Array<number>) {
  const compA = new IntCodeComputer([phases[0], 0], [...args]);
  const compB = new IntCodeComputer([phases[1], compA.output], [...args]);
  const compC = new IntCodeComputer([phases[2], compB.output], [...args]);
  const compD = new IntCodeComputer([phases[3], compC.output], [...args]);
  const compE = new IntCodeComputer([phases[4], compD.output], [...args]);
  compE.printOutput();
  return compE.output;
}

function permute(permutation: Array<number>) {
  var length = permutation.length,
    result = [permutation.slice()],
    c = new Array(length).fill(0),
    i = 1,
    k,
    p;

  while (i < length) {
    if (c[i] < i) {
      k = i % 2 && c[i];
      p = permutation[i];
      permutation[i] = permutation[k];
      permutation[k] = p;
      ++c[i];
      i = 1;
      result.push(permutation.slice());
    } else {
      c[i] = 0;
      ++i;
    }
  }
  return result;
}

const parsedInput = input.value.split(",").map(x => parseInt(x));
const permutations = permute([0, 1, 2, 3, 4]);
const results = [];
for (let i = 0; i < permutations.length; i++) {
  results.push(doIt(permutations[i], parsedInput));
}
console.log(results);
console.log(Math.max(...results));
