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
  values: Array<number>;
  indices: Array<number>;

  constructor(opcode: number) {
    this.parseInput(opcode);
    this.values = [];
    this.indices = [];
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

  parseOpcode(start: number, n: number, codes: Array<number>) {
    for (let i = 0; i < n; i++) {
      let index = codes[start + i + 1];
      this.indices.push(index);
      this.values.push(this.immediate[i] ? index : codes[index]);
    }
  }
}

export default class IntCodeComputer {
  codes: Array<number>;
  isDead: Boolean;
  name: String;
  sp: number;
  inputs: Array<number>;
  outputs: Array<number>;

  constructor(name: string, codes: Array<number>, inputs: Array<number>) {
    this.name = name;
    this.codes = codes;
    this.isDead = false;
    this.sp = 0;
    this.inputs = inputs;
    this.outputs = [];
  }

  printAll() {
    console.log({
      ...this,
      codes: this.codes.join(",")
    });
  }

  handleAddition(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes);
    this.codes[command.indices[2]] = command.values[0] + command.values[1];
    this.sp = start + 4;
    return 0;
  }

  handleMultiplication(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes);
    this.codes[command.indices[2]] = command.values[0] * command.values[1];
    this.sp = start + 4;
    return 0;
  }

  handleInput(start: number, command: Command) {
    if (this.inputs.length === 0) {
      return -1;
    }
    command.parseOpcode(start, 1, this.codes);
    this.codes[command.indices[0]] = this.inputs.shift();
    this.sp = start + 2;
    return 0;
  }

  handleOutput(start: number, command: Command) {
    command.parseOpcode(start, 1, this.codes);
    this.outputs.push(this.codes[command.indices[0]]);

    this.sp = start + 2;
    return 0;
  }

  handleJumpIfTrue(start: number, command: Command) {
    command.parseOpcode(start, 2, this.codes);

    if (command.values[0] !== 0) {
      this.sp = command.values[1];
      return 0;
    }
    this.sp = start + 3;
    return 0;
  }

  handleJumpIfFalse(start: number, command: Command) {
    command.parseOpcode(start, 2, this.codes);

    if (command.values[0] === 0) {
      this.sp = command.values[1];
      return 0;
    }
    this.sp = start + 3;
    return 0;
  }

  handleLessThan(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes);

    if (command.values[0] < command.values[1]) {
      this.codes[command.indices[2]] = 1;
    } else {
      this.codes[command.indices[2]] = 0;
    }

    this.sp = start + 4;
    return 0;
  }

  handleEquals(start: number, command: Command) {
    command.parseOpcode(start, 3, this.codes);

    if (command.values[0] == command.values[1]) {
      this.codes[command.indices[2]] = 1;
    } else {
      this.codes[command.indices[2]] = 0;
    }

    this.sp = start + 4;
    return 0;
  }

  printOutput() {
    console.log("--------------\nOUTPUT: ", this.outputs, "\n--------------");
  }

  processList() {
    while (this.sp < this.codes.length) {
      const command = new Command(this.codes[this.sp]);
      let result;
      switch (command.commandType) {
        case CommandType.ADDITION:
          result = this.handleAddition(this.sp, command);
          break;
        case CommandType.MULTIPLICATION:
          result = this.handleMultiplication(this.sp, command);
          break;
        case CommandType.SAVE:
          result = this.handleInput(this.sp, command);
          break;
        case CommandType.OUTPUT:
          result = this.handleOutput(this.sp, command);
          break;
        case CommandType.JUMP_IF_TRUE:
          result = this.handleJumpIfTrue(this.sp, command);
          break;
        case CommandType.JUMP_IF_FALSE:
          result = this.handleJumpIfFalse(this.sp, command);
          break;
        case CommandType.LESS_THAN:
          result = this.handleLessThan(this.sp, command);
          break;
        case CommandType.EQUALS:
          result = this.handleEquals(this.sp, command);
          break;
        default:
          //EXIT = 99 but it's 9 :(
          //   console.log("died at", this.codes[this.sp], "sp: ", this.sp);
          this.isDead = true;
          return;
      }

      if (result === -1) {
        return;
      }
    }
  }
}

export { IntCodeComputer };
